import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import WalletDetail from "../../src/pages/WalletDetail";

describe("WalletDetail", () => {
  it("provides landmark and heading structure for transaction-heavy wallet views", () => {
    render(<WalletDetail walletId="wallet_demo" />);

    expect(screen.getByRole("main")).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 1, name: "Wallet Details" })).toBeInTheDocument();
    expect(screen.getByRole("region", { name: "Wallet comparison snapshot" })).toBeInTheDocument();
    expect(screen.getByTestId("wallet-balance-delta-cards")).toBeInTheDocument();
  });
});
