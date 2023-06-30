import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { DstageSolanaContracts } from "../target/types/dstage_solana_contracts";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token'; // IGNORE THESE ERRORS IF ANY
import { idlAddress } from "@project-serum/anchor/dist/cjs/idl";
const { SystemProgram } = anchor.web3;

describe("DstageSolanaContracts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.DstageSolanaContracts as Program<DstageSolanaContracts>;
  const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();


  it("NFT Minting!", async () => {

    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );
    console.log("Token MetaData Account Key", TOKEN_METADATA_PROGRAM_ID);
    
    const lamports: number =
      await program.provider.connection.getMinimumBalanceForRentExemption(
        MINT_SIZE
      );
    const getMetadata = async (
      mint: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const getMasterEdition = async (
      mint: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from("edition"),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const NftTokenAccount = await getAssociatedTokenAddress(
      mintKey.publicKey,
      wallet.publicKey
    );
    console.log("NFT Account: ", NftTokenAccount.toBase58());

    const mint_tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mintKey.publicKey,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
        lamports,
      }),
      createInitializeMintInstruction(
        mintKey.publicKey,
        0,
        wallet.publicKey,
        wallet.publicKey
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        NftTokenAccount,
        wallet.publicKey,
        mintKey.publicKey
      )
    );

    const res = await program.provider.sendAndConfirm(mint_tx, [mintKey]);
    console.log(
      await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
    );

    console.log("Account: ", res);
    console.log("Mint key: ", mintKey.publicKey.toString());
    console.log("User: ", wallet.publicKey.toString());

    const metadataAddress = await getMetadata(mintKey.publicKey);
    const masterEdition = await getMasterEdition(mintKey.publicKey);

    console.log("Metadata address: ", metadataAddress.toBase58());
    console.log("MasterEdition: ", masterEdition.toBase58());

    const tx = await program.methods.mintNft(
      mintKey.publicKey,
      "https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA",
      "NFT Title",
    )
      .accounts({
        mintAuthority: wallet.publicKey,
        mint: mintKey.publicKey,
        tokenAccount: NftTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadata: metadataAddress,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        payer: wallet.publicKey,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        masterEdition: masterEdition,
      },
      )
      .rpc();
    console.log("Your transaction signature", tx);
  });






  // it ("Transfer NFT to new ATA Address", async()=> {


  //   let to_address = anchor.web3.Keypair.generate();

  //   //  Transfer Single NFT to ATA
  //   const lamports: number = 
  //   await program.provider.connection.getMinimumBalanceForRentExemption(
  //     MINT_SIZE
  //   );


  //   // ATA Address where to Transfer NFT

  //   let to_ata = await getAssociatedTokenAddress (
  //     mintKey.publicKey, 
  //     to_address.publicKey, 
  //   );

  //   const create_account_tx = new anchor.web3.Transaction().add(
  //     createAssociatedTokenAccountInstruction (
  //       wallet.publicKey, 
  //       to_ata, 
  //       to_address.publicKey,
  //       mintKey.publicKey,
  //     )
  //   );
  //   await program.provider.sendAndConfirm(create_account_tx, []);

  //   // from ATA Account
  //   const from_ata_account = await getAssociatedTokenAddress (
  //     mintKey.publicKey,
  //     wallet.publicKey
  //   );

  //   const transfer_tx = await program.methods.transferNft().accounts({
  //     from : from_ata_account,
  //     to : to_ata,
  //     fromAuthority: wallet.publicKey,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //   }).rpc();

  //   console.log("Trasnfer Hash is ", transfer_tx);
    
  // });




  // it ("Burn Token", async()=> {

  //   let from_ata = await getAssociatedTokenAddress (
  //     mintKey.publicKey, 
  //     wallet.publicKey
  //   ); 

  //   const burn_trx = await program.methods.burnNft().accounts({
  //     mintAddress: mintKey.publicKey,
  //     from: from_ata,
  //     authority: wallet.publicKey, 
  //     tokenProgram: TOKEN_PROGRAM_ID, 
  //   }).rpc();

  //   console.log("Burn Transaction Hash is ", burn_trx);  
  // });




    it ("Place NFT For Fixed Price ", async()=> {

      let amount = 1000000000;
      const fixed_price = new anchor.BN(amount);
      // Find Account Address where from to transfer NFT
      const [acc_address, num] = await anchor.web3.PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("config"), mintKey.publicKey.toBuffer()], 
        program.programId
      );
      let on_fixed_price_tx = await program.methods.placeNftForFixedPrice(fixed_price).accounts({
        mintKey: mintKey.publicKey,
        nftInfo: acc_address,
        authority: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
      
      let get_nft_status = await program.account.nftInfo.fetch(acc_address);
      console.log("NFT Info's are Here ", get_nft_status.saleState);
      console.log("Place On Fixed Price TX = ", on_fixed_price_tx);

    });



    // it ("Purchase NFT For Fixed Price ", async()=> {

    //   let nft_price = new anchor.BN(1000000000);
      
    //   const [nft_info_address, num] = await anchor.web3.PublicKey.findProgramAddress(
    //     [anchor.utils.bytes.utf8.encode("config"), mintKey.publicKey.toBuffer()], 
    //     program.programId
    //   );
      
    //   // generate wallet for to ATA
    //   let to_wallet = anchor.web3.Keypair.generate();

    //   let airdrop_tx = await provider.connection.requestAirdrop(to_wallet.publicKey, 2000000000);
    //   await provider.connection.confirmTransaction(airdrop_tx);
    //   console.log("Air drop Hash is ", airdrop_tx);
      


    //   // from ATA Account
    //   const from_ata_account = await getAssociatedTokenAddress (
    //     mintKey.publicKey,
    //     wallet.publicKey
    //   );

    //   // from ATA Account
    //   const to_ata_account = await getAssociatedTokenAddress (
    //     mintKey.publicKey,
    //     to_wallet.publicKey
    //   );

    //   console.log("Working");
      
    //   const create_account_txx = new anchor.web3.Transaction().add(
    //     createAssociatedTokenAccountInstruction (
    //       to_wallet.publicKey, 
    //       to_ata_account, 
    //       to_wallet.publicKey,
    //       mintKey.publicKey,
    //     )
    //   );
    //   await program.provider.sendAndConfirm(create_account_txx, [to_wallet]);      
    //   const before_balance = await provider.connection.getBalance(wallet.publicKey);
    //   console.log("Before Balance of NFT Authority", before_balance);
      
    //   let tx = await program.methods.prchaseNftAgainstFixedPrice(nft_price).accounts({
    //     mintKey: mintKey.publicKey,
    //     nftInfo: nft_info_address,
    //     // from ATA
    //     fromAta: from_ata_account, 
    //     toAta: to_ata_account,
    //     nftAuthority: wallet.publicKey,
    //     pricePayer: to_wallet.publicKey,
    //     tokenProgram: TOKEN_PROGRAM_ID,
    //   }).rpc();


    //   const after_balance = await provider.connection.getBalance(wallet.publicKey);
    //   // console.log("Before Balance of NFT Authority", after_balance);
    //   console.log("NFT Transfered");

    // });


    it ("Remove NFT From Sale ", async()=> {
      const [nft_info_address, num] = await anchor.web3.PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("config"), mintKey.publicKey.toBuffer()], 
        program.programId
        );
        let tx = await program.methods.removeNftFromSale().accounts({
          mintKey: mintKey.publicKey,
          authority: wallet.publicKey,
          nftInfo: nft_info_address,
        }).rpc();

        let get_nft_status = await program.account.nftInfo.fetch(nft_info_address);
        console.log("NFT State (Remove from Sale) ", get_nft_status.saleState);
        console.log("Remove NFT From Sale is ", tx);
        
    });


    it ("Place NFT For Timed Auction", async()=> {
      
      
      const [nft_info_address, num] = await anchor.web3.PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("config"), mintKey.publicKey.toBuffer()], 
        program.programId
        );

        let auction_start_time = new anchor.BN(1664418155);
        let auction_end_time = new anchor.BN(1664504555);
        let min_bet_amount = new anchor.BN(500000000);
                
        let tx = await program.methods.placeNftForTimedAuction(auction_start_time, auction_end_time, min_bet_amount ).accounts({
        nftInfo: nft_info_address, 
        mintKey: mintKey.publicKey,
        // NFT Minter Authority
        authority: wallet.publicKey, 
      }).rpc();
      console.log("Tx is ", tx);

      let get_nft_status = await program.account.nftInfo.fetch(nft_info_address);
        console.log("NFT State (Remove from Sale) ", get_nft_status.saleState);
        console.log("NFT Placed For Timed Auction ", tx);

    });

    


});
