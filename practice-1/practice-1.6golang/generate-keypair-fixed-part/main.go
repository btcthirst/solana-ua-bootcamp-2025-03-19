package main

import (
	"fmt"
	"regexp"
	"time"

	"github.com/gagliardetto/solana-go"
)

func main() {
	search := `^Kume`
	fmt.Println("Generating keypair with a fixed part in the public key...starting with", search)
	keypair := generateKeypair(search)
	println("Public Key:", keypair.PublicKey().String())
	println("Private Key:", keypair.PrivateKey.String())
}

func generateKeypair(search string) *solana.Wallet {
	start := time.Now()
	var keypair *solana.Wallet
	for {
		keypair = solana.NewWallet()
		fmt.Println("...")
		if bl, _ := regexp.Match(search, []byte(keypair.PublicKey().String())); bl {
			break
		}
	}
	fmt.Printf("\n\nKeypair generated in %v \n\n", time.Since(start))
	return keypair
}
