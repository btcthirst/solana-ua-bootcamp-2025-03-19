package main

import (
	"github.com/gagliardetto/solana-go"
)

func main() {
	// Generate a new keypair
	keypair := solana.NewWallet()

	// Print the public key
	println("Public Key:", keypair.PublicKey().String())

	// Print the private key
	println("Private Key:", keypair.PrivateKey.String())
}
