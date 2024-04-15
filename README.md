### Solana Mixer

#### Introduction
Solana Mixer enhances privacy on the Solana blockchain by mixing fixed amounts of SOL to anonymize transactions. This README outlines the steps to install the Solana CLI, build, and deploy the Solana Mixer program.

#### Prerequisites
- Install [Node.js](https://nodejs.org/)
- Install [Rust and Cargo](https://www.rust-lang.org/tools/install)
- Install [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)

#### Installation

1. **Install Solana CLI (Only for MacOS and Linux)**
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"
   ```

2. **Set Solana to use the Devnet**
   ```bash
   solana config set --url devnet
   ```

3. **Clone the Repository**
   ```bash
   git clone https://github.com/albertoslavicadev/solana-mixer.git
   cd Solana-Mixer
   ```

4. **Build the Program**
   ```bash
   cargo build-bpf --bpf-out-dir=dist/program
   ```

#### Deployment

1. **Generate a new keypair for your program (if you don't have one)**
   ```bash
   solana-keygen new --outfile dist/program-keypair.json
   ```

2. **Deploy the Program**
   ```bash
   solana program deploy dist/program/your_program.so --keypair dist/program-keypair.json
   ```

#### Usage
After deployment, take note of the program ID output by the deployment command. You will need it to interact with your program.

#### Interacting with the Program
1. **Fund Your Wallet**
   ```bash
   solana airdrop 2 $(solana-keygen pubkey dist/program-keypair.json)
   ```

2. **Run a Sample Script**
   - Modify `sample_interaction.js` with your program ID and specific instructions based on your program's API.
   ```bash
   node sample_interaction.js
   ```

#### Support
For support, please open an issue on the GitHub repository.

#### Contribution
Contributions are welcome! Please fork the repository and submit a pull request with your enhancements.

#### License
Solana Mixer is open source and available under the MIT License.
