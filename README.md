# AUTODOX
* this project is owned as an NFT by Ali Husam Shakir AlKaraawi. You can have shares of it by participating to the project.

<img align="left" src="readmeFiles/logo2.png" height="100%">
AUTODOX or AUTODOX not decieded about the name yet.

This app is Notion.so clone, roamresearch clone and obsidian clone. The main purpose of this app is not to just clone these note taking apps but to make an all in one **Open source note taking app** with automation features.
## Goals
1. **Plugins** : plugins or extensions are customizations that you can add to your AUTODOX application. For example, you can add grammar correction plugin like grammarly, or a machine learning plugin that help you abbreviation your text.
2. **Components** : In notion you may noticed that you can import a table when you hit `/` then type table then hit enter. The table called a component. In AUTODOX you can create your own custom components. For example you can create flash cards. Also, you can use plugins to enhance your components. For example, you can use google translator plugin with flash card component so everytime you add a word you will get automatic translation.
3. **Services** : the is the core goal of AUTODOX where you can create a google translator plugin and create flashcard components then put them all in one workspace (page) and you can publish that page so other people can use it. In other word you don't need to create new plugin and component for every user, instead one user can create all of them and share it with others.
3. **search** : We will have 6 search functionaries.
    - search for words in file
    - search for files names
    - global search for words in any files in any directory you choose
    - regular expression search.
    - save your search results and reuse them again.

5. **spreadsheet** : similar to microsoft excel you will have a spreadsheet where you can store your data and implement formulas. Also, with plugins you can implement custom formulas like a stock market plugin. Last but not least, you can use these spreadsheets as a backend for your services. In other word the components will act as a frontend that interact with this spreadsheet.
5. **Ownership** : when you create a component, or a plugin or a service you will own it as an NFT. Hence, you can make money from it. There are three ways to make money with NFTs. One by selling it. Second, by getting percentage on every sell. Third, by requiring subscriptions fees like 7$ a month without selling the plugin or the service.
6. **benefits and vision** :
6.1 First of all, i believe this new system will replace microsoft office and apple iwork.
6.2 Users, will have safe place to store their data on the blockchain on IC. 6.3 Users can do whatever they can imagine with all these customizations.
6.4 there are more feature that I will work one like live-time connection so you can share your documents and update them in real time. or like page components so you can have an entire page as a spreadsheet.


- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)


## Running the project locally
### configurations
1. check [tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/)
    especially the following prerequisites.
    1. $`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
        - or run $`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
    1. install xcode
       - to check that run $`xcode-select --install`
    1. make sure to have c++ 

### running the project
1. $`cd Desktop`
1. $`git clone https://github.com/aliscie/autodox-tauri`
1. $`cd autodox-tauri`
1. $`cargo tauri dev`