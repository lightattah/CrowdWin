# CrowdWin - Decentralized Contest Platform on Solana

**CrowdWin** is a Web3 application built on the Solana blockchain that allows users to create and participate in crowdfunded contests. 

**Key Features:**

* **Contest Creation:**  Users can create contests with custom rules, prize amounts (in SOL), and entry submission formats.
* **Crowdfunding:** Contests are funded by the community. Anyone can contribute SOL to the prize pool.
* **Weighted Voting:**  Contributors (funders) vote for their favourite entries. Voting weight is determined by the percentage of the total prize pool contributed by each user.
* **Decentralized and Transparent:** All contest details, funding, and voting data are stored on the Solana blockchain, ensuring transparency and immutability.
* **Secure and Trustless:**  Leveraging the security of the Solana blockchain, CrowdWin provides a secure platform for managing contests and funds.

**Technology Stack:**

* **Frontend:** React
* **Backend:** Python with Django
* **Smart Contract:** Rust with Anchor
* **Database:** PostgreSQL
* **Solana SDK:** Solana Web3.js
* **Wallet Adapter:** Phantom

**How it Works:**

1. **Create a Contest:** Users define contest parameters, including the prize amount, duration, and entry submission format.
2. **Crowdfund the Prize:** The community contributes SOL to fund the contest. However, contests can be started by a single user or project, and then other users and projects can add to the prize pool to support the contest and gain voting rights as a result.
3. **Submit Entries:** Participants submit their entries according to the contest rules.
4. **Vote for Winners:** Contributors vote for their favourite entries. Votes are weighted based on the percentage of the prize pool contributed.
5. **Winner Determination:** The winner is automatically determined based on the weighted votes.
6. **Prize Distribution:** The prize (in SOL) is automatically transferred to the winner's Solana wallet.

**Getting Started:**

1. **Clone the repository:** `git clone https://github.com/your-username/CrowdWin.git`
2. **Install dependencies:** `npm install` (for frontend and backend)
3. **Set up Solana development environment:** Install Solana CLI, Anchor, and a Solana wallet (Phantom).
4. **Configure environment variables:** Create `.env` files for frontend and backend with required API keys and configurations.
5. **Build and deploy:** Follow the instructions in the `deployment` directory.

**Contributing:**

We welcome contributions to CrowdWin! Please see the `CONTRIBUTING.md` file for guidelines.

**License:**

This project is licensed under the MIT License - see the `LICENSE` file for details.

**Contact:**

* Website: crowdwin.xyz/contactus
* Twitter: x.com/CrowdWinXYZ
* Telegram: t.me/CrowdWinXYZ

**Disclaimer:**

This project is under development. Use at your own risk.
