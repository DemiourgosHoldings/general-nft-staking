require("dotenv").config();
const { promises } = require("fs");
const {
  ApiNetworkProvider,
  ProxyNetworkProvider,
} = require("@multiversx/sdk-network-providers");
const { parseUserKey, UserSigner } = require("@multiversx/sdk-wallet");
const {
  Account,
  ResultsParser,
  AbiRegistry,
  SmartContract,
  Address,
  TransactionWatcher,
  Code,
  CodeMetadata,
  TokenIdentifierValue,
} = require("@multiversx/sdk-core");
const config = require("./config.json");

// load pem content and account
// sign and send tx
// automatic load network and pem files
// provide deploy, upgrade functions
const loadNetworkConfig = () => {
  const workingEnv = process.env.ENVIRONMENT;
  const chainId = process.env[`NETWORKS_${workingEnv}_CHAIN`];
  const gateway = process.env[`NETWORKS_${workingEnv}_GATEWAY`];
  const api = process.env[`NETWORKS_${workingEnv}_API`];
  const pem = process.env[`PEM_${workingEnv}`];

  return {
    chain: chainId,
    gateway,
    api,
    pem,
  };
};

const networkCfg = loadNetworkConfig();

const getPemAndAccount = async () => {
  console.log(`Loading pem and account for ${process.env.ENVIRONMENT}...`);
  console.log(`PEM Path: ${networkCfg.pem}`);
  const apiProvider = new ApiNetworkProvider(networkCfg.api);
  const pemContent = await loadPemContent(networkCfg.pem);
  const account = await loadUserAccount(apiProvider, pemContent);
  return {
    pem: pemContent,
    account,
  };
};

const loadPemContent = async (path) => {
  let buffer = await promises.readFile(path);
  return buffer.toString();
};

const loadUserAccount = async (apiProvider, walletPemContents) => {
  const userKey = parseUserKey(walletPemContents);
  const address = userKey.generatePublicKey().toAddress();

  const account = new Account(address);
  const apiAccount = await apiProvider.getAccount(address);
  account.update(apiAccount);
  return account;
};

const Parser = new ResultsParser();

const getSmartContract = async () => {
  const scAddress = config.address[process.env.ENVIRONMENT];
  const abiJson = await promises.readFile(process.env.SC_ABI_FILE_PATH, {
    encoding: "utf8",
  });
  const abiObj = JSON.parse(abiJson);
  const abiRegistry = AbiRegistry.create(abiObj);
  return new SmartContract({
    address: new Address(scAddress),
    abi: abiRegistry,
  });
};

const signAndSend = async (tx, walletPemContents) => {
  const provider = getProxyProvider();
  const signer = prepareUserSigner(walletPemContents);
  const serializedTransaction = tx.serializeForSigning();
  const signature = await signer.sign(serializedTransaction);
  tx.applySignature(signature);
  await provider.sendTransaction(tx);
  console.log(`Transaction sent. Tx hash: ${tx.getHash().toString()}`);
  const watcher = new TransactionWatcher(provider);
  const transactionOnNetwork = await watcher.awaitCompleted(tx);
  return transactionOnNetwork;
};

const prepareUserSigner = (walletPemContents) => {
  return UserSigner.fromPem(walletPemContents);
};

const getProxyProvider = () => {
  return new ProxyNetworkProvider(networkCfg.gateway);
};

const createCodeMetadata = (payable, payableBySc) => {
  return new CodeMetadata(true, true, payable, payableBySc);
};

const loadWasm = async () => {
  let buffer = await promises.readFile(process.env.SC_WASM_FILE_PATH);
  let code = Code.fromBuffer(buffer);
  return code;
};

const deploy = async () => {
  let contract = new SmartContract();
  let { pem, account } = await getPemAndAccount();
  let code = await loadWasm();
  let codeMetadata = createCodeMetadata(
    config.deploymentArgs.payable,
    config.deploymentArgs.payableBySc
  );

  const transaction = contract.deploy({
    deployer: account.address,
    code: code,
    codeMetadata: codeMetadata,
    initArguments: buildDeployArgs(),
    gasLimit: config.deploymentArgs.gasLimit,
    chainID: networkCfg.chain,
  });
  transaction.setNonce(account.getNonceThenIncrement());

  console.log(`Deploying contract on ${process.env.ENVIRONMENT}...`);
  const txResult = await signAndSend(transaction, pem);
  const deployedAddress = deploymentTransactionResultHandler(txResult);

  if (deployedAddress !== "") {
    config.address[process.env.ENVIRONMENT] = deployedAddress;
    await promises.writeFile("./config.json", JSON.stringify(config, null, 2));
  }
  console.log(`Deployment completed. Contract address: ${deployedAddress}`);
  return deployedAddress;
};

const upgrade = async () => {
  let address = config.address[process.env.ENVIRONMENT];
  if (!address) {
    console.log("Contract address not found. Please deploy first.");
    return;
  }
  let contract = await getSmartContract();
  let { pem, account } = await getPemAndAccount();
  let code = await loadWasm();
  let codeMetadata = createCodeMetadata(
    config.deploymentArgs.payable,
    config.deploymentArgs.payableBySc
  );

  const transaction = contract.upgrade({
    caller: account.address,
    code: code,
    codeMetadata: codeMetadata,
    initArguments: buildDeployArgs(),
    gasLimit: config.deploymentArgs.gasLimit,
    chainID: networkCfg.chain,
  });
  transaction.setNonce(account.getNonceThenIncrement());

  console.log(`Upgrading contract on ${process.env.ENVIRONMENT}...`);
  const txResult = await signAndSend(transaction, pem);
  const deployedAddress = deploymentTransactionResultHandler(txResult);

  if (deployedAddress !== "") {
    config.address[process.env.ENVIRONMENT] = deployedAddress;
    await promises.writeFile("./config.json", JSON.stringify(config, null, 2));
  }
  console.log(`Upgrade completed. Contract address: ${deployedAddress}`);
  return deployedAddress;
};

const deploymentTransactionResultHandler = (transactionResult) => {
  if (transactionResult.status.status !== "success") {
    console.log("Transaction failed", transactionResult);
    return "";
  } else {
    console.log(
      "Deployment successful. Contract address: ",
      transactionResult.logs.events[0].address.value
    );
    return transactionResult.logs.events[0].address.value;
  }
};

const buildDeployArgs = () => {
  const args = [];
  config.deploymentArgs[process.env.ENVIRONMENT].forEach((arg) => {
    switch (arg.type) {
      case "TokenIdentifier":
        args.push(new TokenIdentifierValue(arg.value));
        break;
    }
  });
  return args;
};

module.exports = {
  loadNetworkConfig,
  getPemAndAccount,
  getSmartContract,
  resultsParser: Parser,
  signAndSend,
  getProxyProvider,

  deploy,
  upgrade,
};
