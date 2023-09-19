require("dotenv").config();
const { promises, stat } = require("fs");
const config = require("./config.json");
const {
  deploy,
  upgrade,
  getSmartContract,
  signAndSendTx,
} = require("./common");
let chalk;

import("chalk").then((module) => {
  chalk = module.default;
});

const stakingModuleTypes = [
  "CodingDivisionSfts",
  "Bloodshed",
  "Nosferatu",
  "VestaXDAO",
  "SnakesSfts",
  "SharesSfts",
  "XBunnies",
];

const readline = require("readline");
const {
  TokenIdentifierValue,
  U8Value,
  U32Value,
  U64Value,
} = require("@multiversx/sdk-core/out");

const run = () => {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question(
    "Please choose an option: \n 1. Deploy \n 2. Upgrade \n 3. Register new staking pool \n 4. Configure pool scores\n> ",
    async (option) => {
      switch (option) {
        case "1":
          await deploy();
          rl.close();
          break;
        case "2":
          await upgrade();
          rl.close();
          break;
        case "3":
          rl.question(
            "Enter NFT/SFT collection token identifier: ",
            async (tokenIdentifier) => {
              console.log("Please select a staking module type: ");
              stakingModuleTypes.forEach((moduleType, index) =>
                console.log(`${index + 1}. ${moduleType}`)
              );
              rl.question(
                "Enter the number corresponding to the module type: ",
                async (moduleType) => {
                  await handleRegisterNewStakingPool(
                    tokenIdentifier,
                    moduleType
                  );
                  rl.close();
                }
              );
            }
          );
          break;
        case "4":
          rl.question(".CSV configuration file path: ", async (filePath) => {
            rl.question(
              "Does .CSV file include headers? (Y/N): ",
              async (includesHeaders) => {
                const hasHeaders = includesHeaders.toLowerCase() === "y";
                await handleConfigurePoolScores(filePath, hasHeaders);
                rl.close();
              }
            );
          });
          break;
        default:
          console.log("Invalid option");
          rl.close();
      }
    }
  );
};

const handleRegisterNewStakingPool = async (tokenIdentifier, moduleType) => {
  let message = chalk.white("Registering new staking pool for ");
  message += chalk.yellow(tokenIdentifier);
  message += chalk.white(" with module type ");
  message += chalk.yellow(stakingModuleTypes[moduleType - 1]);
  message += chalk.white("... ");

  process.stdout.write(message + "\n");

  let contract = await getSmartContract();
  let tx = contract.methodsExplicit
    .createPool([
      new TokenIdentifierValue(tokenIdentifier),
      new U8Value(moduleType),
    ])
    .withChainID("D")
    .withGasLimit(15_000_000);

  let status = await signAndSendTx(tx);

  message += status ? chalk.green("SUCCESS") : chalk.red("FAILED");
  process.stdout.write(message + "\n");
};

const handleConfigurePoolScores = async (filePath, hasHeaders) => {
  let fileContent = await promises.readFile(filePath, "utf8");
  const lines = fileContent
    .split("\n")
    .filter((_, index) => (hasHeaders ? index > 0 : true))
    .map((line) => line.trim());
  console.log(`${lines.length} transactions to be sent`);
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    try {
      await handleConfigureScore(line);
    } catch (error) {
      console.error(`Error processing line ${i}: ${error}`);
    }
  }
};

const handleConfigureScore = async (row) => {
  let columns = row.split(",");

  let tokenIdentifier = columns[0];
  let nonces = columns[1].length === 0 ? [] : columns[1].split(";");
  let nonceRangeStart = columns[2] === "" ? undefined : columns[2];
  let nonceRangeEnd = columns[3] === "" ? undefined : columns[3];
  let score = columns[4];
  let fullSetBonusScore = columns[5] === "" ? undefined : columns[5];

  process.stdout.write(
    getPoolRowMessage(
      tokenIdentifier,
      nonces,
      nonceRangeStart,
      nonceRangeEnd,
      score,
      fullSetBonusScore
    )
  );
  process.stdout.write("\n");

  let status = false;
  if (
    nonces.length === 0 &&
    nonceRangeStart === undefined &&
    nonceRangeEnd === undefined
  ) {
    status = await sendSetBaseAssetScoreTx(tokenIdentifier, score);
  }
  if (nonces.length > 0) {
    status = await sendSetNonceAssetScoreTx(tokenIdentifier, nonces, score);
  }
  if (nonceRangeStart !== undefined && nonceRangeEnd !== undefined) {
    status = await sendSetNonceAssetScoreByRangeTx(
      tokenIdentifier,
      nonceRangeStart,
      nonceRangeEnd,
      score
    );
  }

  if (fullSetBonusScore !== undefined) {
    let fullSetScoreResult = await sendSetFullSetBonusScoreTx(
      tokenIdentifier,
      fullSetBonusScore
    );
    status = status && fullSetScoreResult;
  }
  // process.stdout.write(status ? chalk.green("SUCCESS") : chalk.red("FAILED"));
  // process.stdout.write("\n");
  process.stdout.write(
    getPoolRowMessage(
      tokenIdentifier,
      nonces,
      nonceRangeStart,
      nonceRangeEnd,
      score,
      fullSetBonusScore,
      status
    ) + "\n"
  );
};

const sendSetBaseAssetScoreTx = async (tokenIdentifier, score) => {
  let contract = await getSmartContract();
  let tx = contract.methodsExplicit
    .setBaseAssetScore([
      new TokenIdentifierValue(tokenIdentifier),
      new U32Value(1),
      new U32Value(score),
    ])
    .withGasLimit(15_000_000);

  return await signAndSendTx(tx);
};

const sendSetNonceAssetScoreTx = async (tokenIdentifier, nonces, score) => {
  let contract = await getSmartContract();
  let noncesArg = nonces.map((nonce) => new U64Value(nonce));
  let tx = contract.methodsExplicit
    .setNonceAssetScore(
      [
        new TokenIdentifierValue(tokenIdentifier),
        new U8Value(1),
        new U32Value(score),
      ].concat(noncesArg)
    )
    .withGasLimit(15_000_000 + nonces.length * 100_000);

  return await signAndSendTx(tx);
};

const sendSetNonceAssetScoreByRangeTx = async (
  tokenIdentifier,
  nonceRangeStart,
  nonceRangeEnd,
  score
) => {
  let contract = await getSmartContract();
  let gasLimit = 15_000_000 + (nonceRangeEnd - nonceRangeStart) * 100_000;
  let tx = contract.methodsExplicit
    .setNonceAssetScoreByRange([
      new TokenIdentifierValue(tokenIdentifier),
      new U8Value(1),
      new U32Value(score),
      new U64Value(nonceRangeStart),
      new U64Value(nonceRangeEnd),
    ])
    .withGasLimit(gasLimit > 600_000_000 ? 600_000_000 : gasLimit);

  return await signAndSendTx(tx);
};

const sendSetFullSetBonusScoreTx = async (tokenIdentifier, score) => {
  let contract = await getSmartContract();
  let tx = contract.methodsExplicit
    .setFullSetScore([
      new TokenIdentifierValue(tokenIdentifier),
      new U8Value(1),
      new U32Value(score),
    ])
    .withGasLimit(15_000_000);

  return await signAndSendTx(tx);
};

const getPoolRowMessage = (
  tokenIdentifier,
  nonces,
  nonceRangeStart,
  nonceRangeEnd,
  score,
  fullSetBonusScore,
  statusSuccessful
) => {
  let statusMessage = chalk.white("Setting up ");
  statusMessage += chalk.yellow(tokenIdentifier);
  if (nonces.length > 0) {
    statusMessage += chalk.white(" for ");
    statusMessage += chalk.yellow(nonces.join(", "));
    statusMessage += chalk.white(" nonces");
  }
  if (nonceRangeStart !== undefined && nonceRangeEnd !== undefined) {
    statusMessage += chalk.white(" for nonces between ");
    statusMessage += chalk.yellow(nonceRangeStart);
    statusMessage += chalk.white(" and ");
    statusMessage += chalk.yellow(nonceRangeEnd);
  }

  statusMessage += chalk.white(" with score ");
  statusMessage += chalk.yellow(score);
  if (fullSetBonusScore !== undefined) {
    statusMessage += chalk.white(" and full set bonus score ");
    statusMessage += chalk.yellow(fullSetBonusScore);
  }
  statusMessage += chalk.white("... ");

  if (statusSuccessful === undefined) {
    return statusMessage;
  }

  if (statusSuccessful) {
    statusMessage += chalk.green("SUCCESS");
  } else {
    statusMessage += chalk.red("FAILED");
  }

  return statusMessage;
};

run();
