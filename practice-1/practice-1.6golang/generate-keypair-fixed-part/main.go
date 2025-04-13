package main

import (
	"fmt"
	"regexp"
	"strings"
	"time"

	"github.com/gagliardetto/solana-go"
)

func main() {
	search := `^anza`
	fmt.Println("Generating keypair with a fixed part in the public key...match", search)
	keypair := generateKeypair(search)
	println("Public Key:", keypair.PublicKey().String())
	println("Private Key:", keypair.PrivateKey.String())
}

func generateKeypair(search string) *solana.Wallet {
	start := time.Now()
	var keypair *solana.Wallet
	for {
		keypair = solana.NewWallet()
		pub := strings.ToLower(keypair.PublicKey().String())
		if bl, _ := regexp.Match(search, []byte(pub)); bl {
			break
		}
	}
	fmt.Printf("\n\nKeypair generated in %v \n\n", time.Since(start))
	return keypair
}
