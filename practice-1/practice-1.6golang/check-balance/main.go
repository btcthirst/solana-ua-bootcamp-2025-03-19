package main

import (
	"context"
	"fmt"
	"os"

	"github.com/gagliardetto/solana-go"
	"github.com/gagliardetto/solana-go/rpc"
	"github.com/joho/godotenv"
)

func main() {
	secretKey, err := loadSecretFromEnv()
	if err != nil {
		panic(err)
	}

	privatKey, err := privatKeyFromSecret(secretKey)
	if err != nil {
		panic(err)
	}
	client := rpc.New(rpc.DevNet_RPC)
	balance, err := client.GetBalance(context.Background(), privatKey.PublicKey(), rpc.CommitmentFinalized)
	if err != nil {
		panic(err)
	}
	fmt.Printf("💰 The balance for the wallet at address %s is: %v SOL", privatKey.PublicKey().String(), balance.Value/solana.LAMPORTS_PER_SOL)
}

func loadSecretFromEnv() (string, error) {
	err := godotenv.Load()
	if err != nil {
		return "", err
	}
	secretKey := os.Getenv("SECRET_KEY")
	if secretKey == "" {
		return "", os.ErrNotExist
	}

	return secretKey, nil
}

func privatKeyFromSecret(secretKey string) (solana.PrivateKey, error) {
	privatKey := solana.MustPrivateKeyFromBase58(secretKey)
	if privatKey == nil {
		return nil, os.ErrInvalid
	}
	return privatKey, nil
}
