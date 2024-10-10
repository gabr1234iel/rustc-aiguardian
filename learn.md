Certainly! To use your Solana programs in a frontend application, you'll typically use the Solana Web3.js library along with Anchor's JavaScript library. Here's a general guide on how to integrate these programs into a frontend:



1. Set up your project:
   First, install the necessary dependencies:

   ```bash
   npm install @solana/web3.js @project-serum/anchor
   ```

2. Connect to Solana:
   Create a connection to the Solana network:

   ```javascript
   import { Connection, PublicKey } from '@solana/web3.js';
   import { Program, AnchorProvider } from '@project-serum/anchor';

   const connection = new Connection('https://api.devnet.solana.com');
   ```

3. Set up the wallet adapter:
   You'll need a wallet adapter to interact with the user's wallet. For example, you might use Phantom:

   ```javascript
   import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';

   const wallet = new PhantomWalletAdapter();
   ```

4. Create the Anchor provider:

   ```javascript
   const provider = new AnchorProvider(connection, wallet, {
     preflightCommitment: 'processed',
   });
   ```

5. Load your program:
   For each of your programs, you'll need to load the IDL (Interface Description Language) file and create a Program instance:

   ```javascript
   import idl from './idl/decentralized_post.json';

   const programId = new PublicKey('AfCDBjnYCyHh7Hb9YiKx8NVQXA7dFfaaY5yFFF8DabJb');
   const program = new Program(idl, programId, provider);
   ```

6. Interact with your programs:
   Now you can call your program's instructions. Here's an example for the `create_post` instruction:

   ```javascript
   async function createPost(ipfsHash, imageHash, content, worldId) {
     try {
       const tx = await program.methods.createPost(ipfsHash, imageHash, content, worldId)
         .accounts({
           postAccount: // PDA for post account
           user: provider.wallet.publicKey,
         })
         .rpc();
       console.log("Transaction signature", tx);
     } catch (error) {
       console.error("Error creating post:", error);
     }
   }
   ```

   Similarly, for the `deepfake_storage` program:

   ```javascript
   async function storeImage(imageHash, deepfakeValue) {
     try {
       const tx = await deepfakeProgram.methods.storeImage(imageHash, deepfakeValue)
         .accounts({
           deepfakeAccount: // PDA for deepfake account
           user: provider.wallet.publicKey,
         })
         .rpc();
       console.log("Transaction signature", tx);
     } catch (error) {
       console.error("Error storing image:", error);
     }
   }
   ```

7. Fetch data from your programs:
   You can also fetch data from your programs. For example:

   ```javascript
   async function getPost(postId) {
     try {
       const post = await program.account.postAccount.fetch(postId);
       console.log("Post data:", post);
     } catch (error) {
       console.error("Error fetching post:", error);
     }
   }
   ```

8. Handle program events:
   You can listen for program events:

   ```javascript
   program.addEventListener('PostCreated', (event, slot) => {
     console.log('New post created:', event.postId);
   });
   ```

Remember to replace placeholder values (like PDAs) with actual account addresses as needed.

Here's a more complete example putting it all together:

```javascript
import { Connection, PublicKey } from '@solana/web3.js';
import { Program, AnchorProvider } from '@project-serum/anchor';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import decentralizedPostIdl from './idl/decentralized_post.json';
import deepfakeStorageIdl from './idl/deepfake_storage.json';
import originalityStorageIdl from './idl/originality_storage.json';

// Initialize connection to Solana devnet
const connection = new Connection('https://api.devnet.solana.com');

// Set up wallet adapter
const wallet = new PhantomWalletAdapter();

// Create Anchor provider
const provider = new AnchorProvider(connection, wallet, {
  preflightCommitment: 'processed',
});

// Initialize programs
const decentralizedPostProgram = new Program(decentralizedPostIdl, new PublicKey('AfCDBjnYCyHh7Hb9YiKx8NVQXA7dFfaaY5yFFF8DabJb'), provider);
const deepfakeStorageProgram = new Program(deepfakeStorageIdl, new PublicKey('Aqyqt3mnUVMDErUPvQm9e4LDWHHtJKpLXsBhkumbk6L2'), provider);
const originalityStorageProgram = new Program(originalityStorageIdl, new PublicKey('4CAZ7URST3D1yMU968iZtEEerN4TCZW2eKDvDWqHSZvE'), provider);

// Function to create a post
async function createPost(ipfsHash, imageHash, content, worldId) {
  try {
    const tx = await decentralizedPostProgram.methods.createPost(ipfsHash, imageHash, content, worldId)
      .accounts({
        postAccount: await PublicKey.findProgramAddress([Buffer.from('post')], decentralizedPostProgram.programId),
        user: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Post created. Transaction signature:", tx);
  } catch (error) {
    console.error("Error creating post:", error);
  }
}

// Function to store image deepfake info
async function storeImageDeepfakeInfo(imageHash, deepfakeValue) {
  try {
    const tx = await deepfakeStorageProgram.methods.storeImage(imageHash, deepfakeValue)
      .accounts({
        deepfakeAccount: await PublicKey.findProgramAddress([Buffer.from('deepfake')], deepfakeStorageProgram.programId),
        user: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Deepfake info stored. Transaction signature:", tx);
  } catch (error) {
    console.error("Error storing deepfake info:", error);
  }
}

// Function to store image originality
async function storeImageOriginality(imageHash, originality) {
  try {
    const tx = await originalityStorageProgram.methods.storeOriginality(imageHash, originality)
      .accounts({
        originalityAccount: await PublicKey.findProgramAddress([Buffer.from('originality')], originalityStorageProgram.programId),
        user: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Originality info stored. Transaction signature:", tx);
  } catch (error) {
    console.error("Error storing originality info:", error);
  }
}

// Example usage
createPost("QmXyz...", "hash123", "This is a test post", "world1");
storeImageDeepfakeInfo("hash123", 2);
storeImageOriginality("hash123", true);

// Listen for PostCreated events
decentralizedPostProgram.addEventListener('PostCreated', (event, slot) => {
  console.log('New post created:', event.postId);
});

```

This example provides a basic structure for interacting with your Solana programs from a frontend application. You'll need to adapt this to your specific frontend framework (React, Vue, etc.) and add appropriate error handling and user interface elements.

Remember to handle wallet connections, account creation, and other necessary setup in your actual application. Also, ensure you're following best practices for security when dealing with user wallets and sensitive operations.

Is there a specific part of the frontend integration you'd like me to elaborate on?