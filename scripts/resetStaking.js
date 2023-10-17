const { getSmartContract, signAndSendTx } = require("./common");

const run = async () => {
  const contract = await getSmartContract();
  const payload = [
    // [
    //   "erd1tztluf08g90max7jkr4jtac9w5qv7qacgkhh57q9nz2erq9y2p3sd5njkg",
    //   "DEMIOU-704b5c",
    //   1,
    //   3,
    // ],
    // [
    //   "erd1j43ssgjr8x9m0s7zz0xhn7qdgssdetm86jc6a2dzsegs74fmyl5ssv44c4",
    //   "DHCD-bc9963",
    //   1,
    //   1,
    // ],
    // [
    //   "erd1j43ssgjr8x9m0s7zz0xhn7qdgssdetm86jc6a2dzsegs74fmyl5ssv44c4",
    //   "DHCD-bc9963",
    //   2,
    //   1,
    // ],
    [
      "erd1j43ssgjr8x9m0s7zz0xhn7qdgssdetm86jc6a2dzsegs74fmyl5ssv44c4",
      "VESTAXDAO-e6c48c",
      2,
      2,
    ],
  ];

  for (let i = 0; i < payload.length; i++) {
    const [address, token, nonce, amount] = payload[i];
    await reset(contract, address, token, nonce, amount);
  }
};

const reset = async (contract, address, token, nonce, amount) => {
  const interaction = contract.methods.reset([
    address,
    token,
    nonce,
    amount,
  ]);
  interaction.withGasLimit(15_000_000);

  await signAndSendTx(interaction);
};

run().then(() => process.exit(0));
