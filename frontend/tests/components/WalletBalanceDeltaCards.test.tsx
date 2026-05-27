import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { WalletBalanceDeltaCards } from "../../src/components/v1/WalletBalanceDeltaCards";

describe("WalletBalanceDeltaCards", () => {
  it("renders side-by-side wallet comparison cards", () => {
    render(
      <WalletBalanceDeltaCards
        left={{ id: "left", label: "Primary", currentBalance: 100, previousBalance: 80 }}
        right={{ id: "right", label: "Secondary", currentBalance: 55, previousBalance: 60 }}
      />,
    );

    expect(screen.getByTestId("wallet-balance-delta-cards-left")).toBeInTheDocument();
    expect(screen.getByTestId("wallet-balance-delta-cards-right")).toBeInTheDocument();
    expect(screen.getByText("100.00 XLM")).toBeInTheDocument();
  });

  it("renders fallback content when balances are missing", () => {
    render(
      <WalletBalanceDeltaCards
        left={{ id: "left", label: "Primary", currentBalance: null, previousBalance: null }}
        right={{ id: "right", label: "Secondary", currentBalance: undefined, previousBalance: undefined }}
      />,
    );

    expect(screen.getAllByText("—").length).toBeGreaterThan(1);
  });
});
