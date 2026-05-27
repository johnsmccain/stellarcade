import React from "react";
import { InlineStatDelta } from "./InlineStatDelta";
import "./WalletBalanceDeltaCards.css";

export interface WalletComparisonBalance {
  id: string;
  label: string;
  currentBalance?: number | null;
  previousBalance?: number | null;
}

export interface WalletBalanceDeltaCardsProps {
  left: WalletComparisonBalance;
  right: WalletComparisonBalance;
  loading?: boolean;
  testId?: string;
}

const formatBalance = (value?: number | null): string => {
  if (value === null || value === undefined || Number.isNaN(value)) {
    return "—";
  }
  return `${value.toFixed(2)} XLM`;
};

const balanceDelta = (input: WalletComparisonBalance): number | null => {
  if (input.currentBalance === null || input.currentBalance === undefined) return null;
  if (input.previousBalance === null || input.previousBalance === undefined) return null;
  return input.currentBalance - input.previousBalance;
};

function ComparisonCard({
  balance,
  loading,
  testId,
}: {
  balance: WalletComparisonBalance;
  loading: boolean;
  testId: string;
}) {
  const delta = balanceDelta(balance);
  return (
    <article className="wallet-balance-delta-card" data-testid={testId}>
      <h3 className="wallet-balance-delta-card__title">{balance.label}</h3>
      <p className="wallet-balance-delta-card__amount">
        {loading ? "Loading..." : formatBalance(balance.currentBalance)}
      </p>
      <InlineStatDelta
        value={loading ? null : delta}
        label="vs previous snapshot"
        testId={`${testId}-delta`}
      />
    </article>
  );
}

export const WalletBalanceDeltaCards: React.FC<WalletBalanceDeltaCardsProps> = ({
  left,
  right,
  loading = false,
  testId = "wallet-balance-delta-cards",
}) => {
  return (
    <section
      className="wallet-balance-delta-cards"
      aria-label="Wallet balance comparison"
      data-testid={testId}
    >
      <ComparisonCard balance={left} loading={loading} testId={`${testId}-left`} />
      <ComparisonCard balance={right} loading={loading} testId={`${testId}-right`} />
    </section>
  );
};

export default WalletBalanceDeltaCards;
