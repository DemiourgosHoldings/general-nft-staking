require("dotenv").config();
const config = require("./config.json");
const { loadNetworkConfig, deploy, upgrade } = require("./common");

const readline = require("readline");

const run = () => {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question(
    "Please choose an option: \n 1. Deploy \n 2. Upgrade \n 3. Add Pool \n",
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
          handleAddPools();
          rl.close();
          break;
        default:
          console.log("Invalid option");
      }
    }
  );
};

const handleAddPools = async () => {
  const pools = config.poolSettings[process.env.ENVIRONMENT].pools;
  console.log(pools);
};

run();
