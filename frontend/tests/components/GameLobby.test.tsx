import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, test, vi } from "vitest";
import { commandStore } from "../../src/components/v1/CommandPalette";
import GameLobby from "../../src/pages/GameLobby";
import { ONBOARDING_CHECKLIST_DISMISSED_FLAG } from "../../src/services/global-state-store";
import { ApiClient } from "../../src/services/typed-api-sdk";

vi.mock("../../src/services/typed-api-sdk");

const walletState = vi.hoisted(() => ({
  status: "DISCONNECTED",
  address: null,
  network: null,
  provider: null,
  capabilities: {
    isConnected: false,
    isConnecting: false,
    isReconnecting: false,
    canConnect: true,
  },
  error: null,
  connect: vi.fn(),
  disconnect: vi.fn(),
  refresh: vi.fn(),
  isRefreshing: false,
  lastUpdatedAt: null,
  refreshState: {
    phase: "IDLE",
    trigger: "silent",
    attempt: 0,
    maxAttempts: 1,
    terminal: false,
  },
  sessionDropped: false,
  lastReconnectAt: null as number | null,
}));

vi.mock("../../src/hooks/v1/useWalletStatus", () => ({
  useWalletStatus: () => walletState,
}));

vi.mock("../../src/utils/v1/useNetworkGuard", () => ({
  isSupportedNetwork: () => ({
    isSupported: true,
    normalizedActual: "TESTNET",
    supportedNetworks: ["TESTNET", "PUBLIC"],
  }),
}));

beforeEach(() => {
  localStorage.clear();
  sessionStorage.clear();
  commandStore.dispatch({ type: "COMMAND_PALETTE_CLOSE" });
  walletState.status = "DISCONNECTED";
  walletState.address = null;
  walletState.network = null;
  walletState.provider = null;
  walletState.capabilities = {
    isConnected: false,
    isConnecting: false,
    isReconnecting: false,
    canConnect: true,
  };
  walletState.error = null;
  walletState.connect = vi.fn();
  walletState.disconnect = vi.fn();
  walletState.refresh = vi.fn();
  walletState.isRefreshing = false;
  walletState.lastUpdatedAt = null;
  walletState.refreshState = {
    phase: "IDLE",
    trigger: "silent",
    attempt: 0,
    maxAttempts: 1,
    terminal: false,
  };
  walletState.sessionDropped = false;
  walletState.lastReconnectAt = null;
});

test("renders GameLobby and fetches games", async () => {
  (ApiClient as any).prototype.getGames.mockResolvedValue({
    success: true,
    data: [{ id: "123456789", name: "Elite Clash", status: "active", wager: 50 }],
  });

  render(<GameLobby />);

  expect(screen.getByText(/loading elite games/i)).toBeDefined();

  await waitFor(() => {
    expect(screen.getAllByText(/Elite Clash/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/50 XLM/i).length).toBeGreaterThan(0);
    expect(screen.getByText(/#12345678/i)).toBeDefined();
  });
});

describe("GameLobby", () => {
  it("renders the mission strip for a first-time dashboard session", async () => {
    (ApiClient as any).prototype.getGames.mockResolvedValue({ success: true, data: [] });

    render(<GameLobby />);

    await waitFor(() => {
      expect(screen.getByTestId("dashboard-mission-strip")).toBeInTheDocument();
      expect(screen.getByText(/new dashboard session/i)).toBeInTheDocument();
    });
  });

  it("does not render the mission strip when the dismissed flag is set", async () => {
    localStorage.setItem(
      "stc_global_state_v1",
      JSON.stringify({
        auth: { isAuthenticated: false },
        flags: { [ONBOARDING_CHECKLIST_DISMISSED_FLAG]: true },
        storedAt: Date.now(),
      }),
    );
    (ApiClient as any).prototype.getGames.mockResolvedValue({ success: true, data: [] });

    render(<GameLobby />);

    await waitFor(() => {
      expect(screen.queryByTestId("dashboard-mission-strip")).not.toBeInTheDocument();
    });
  });

  it("marks the command mission complete after using the quick-action surface", async () => {
    (ApiClient as any).prototype.getGames.mockResolvedValue({ success: true, data: [] });

    render(<GameLobby />);

    await waitFor(() => {
      expect(screen.getByTestId("dashboard-mission-strip-learn-commands")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByTestId("quick-action-surface-command-center"));

    expect(commandStore.selectCommandPaletteOpen()).toBe(true);
    expect(screen.getByTestId("dashboard-mission-strip-learn-commands")).toHaveTextContent(/complete/i);
  });

  it("restores compact leaderboard density from persisted preference", async () => {
    localStorage.setItem("stc_table_density_v1_dashboard-surfaces", "compact");
    (ApiClient as any).prototype.getGames.mockResolvedValue({
      success: true,
      data: [{ id: "g1", name: "Game One", status: "active", wager: 25 }],
    });

    render(<GameLobby />);

    await waitFor(() => {
      expect(screen.getByTestId("leaderboard-table")).toHaveClass("data-table--compact");
    });

    expect(screen.getByTestId("leaderboard-density-compact")).toHaveAttribute(
      "aria-pressed",
      "true",
    );
  });

  it("renders queue participation summary widget in live lobby flow", async () => {
    (ApiClient as any).prototype.getGames.mockResolvedValue({
      success: true,
      data: [{ id: "g1", name: "Game One", status: "active", wager: 25 }],
    });

    render(<GameLobby />);

    await waitFor(() => {
      expect(screen.getByTestId("lobby-queue-summary")).toBeInTheDocument();
      expect(screen.getByText(/Queue participation summary/i)).toBeInTheDocument();
    });
  });

  it("prompts the user to return to the last context after reconnecting", async () => {
    sessionStorage.setItem("stc_dashboard_last_context_v1", "activity-rail");
    walletState.status = "RECONNECTING";
    walletState.capabilities = {
      isConnected: false,
      isConnecting: false,
      isReconnecting: true,
      canConnect: false,
    };
    walletState.lastReconnectAt = 1_700_000_000_000;

    (ApiClient as any).prototype.getGames.mockResolvedValue({
      success: true,
      data: [{ id: "g1", name: "Game One", status: "active", wager: 25 }],
    });

    const { rerender } = render(<GameLobby />);

    await waitFor(() => {
      expect(screen.getByText("Live Arena")).toBeInTheDocument();
    });

    walletState.status = "CONNECTED";
    walletState.capabilities = {
      isConnected: true,
      isConnecting: false,
      isReconnecting: false,
      canConnect: false,
    };
    walletState.lastReconnectAt = 1_700_000_000_500;
    rerender(<GameLobby />);

    await waitFor(() => {
      expect(screen.getByTestId("lobby-resume-context-banner")).toHaveTextContent(
        /wallet activity rail/i,
      );
    });

    fireEvent.click(screen.getByTestId("lobby-resume-context-banner-dismiss-btn"));
    expect(screen.queryByTestId("lobby-resume-context-banner")).not.toBeInTheDocument();
  });

  it("shows a pending-action resume chip for interrupted transaction flows", async () => {
    localStorage.setItem(
      "stc_global_state_v1",
      JSON.stringify({
        auth: { isAuthenticated: false },
        flags: {},
        pendingTransaction: {
          operation: "wallet.deposit",
          phase: "SUBMITTING",
          txHash: "abc1234567890",
          startedAt: 1_700_000_000_000,
          updatedAt: 1_700_000_000_500,
        },
        storedAt: Date.now(),
      }),
    );
    (ApiClient as any).prototype.getGames.mockResolvedValue({
      success: true,
      data: [{ id: "g1", name: "Game One", status: "active", wager: 25 }],
    });

    render(<GameLobby />);

    await waitFor(() => {
      expect(screen.getByTestId("lobby-pending-action-chip")).toHaveTextContent(
        /wallet deposit/i,
      );
    });

    fireEvent.click(screen.getByTestId("lobby-pending-action-chip-resume-btn"));
    expect(screen.getByTestId("transaction-detail-drawer")).toBeInTheDocument();
  });

  it("lets the user dismiss the pending-action chip", async () => {
    localStorage.setItem(
      "stc_global_state_v1",
      JSON.stringify({
        auth: { isAuthenticated: false },
        flags: {},
        pendingTransaction: {
          operation: "wallet.deposit",
          phase: "SUBMITTING",
          txHash: "abc1234567890",
          startedAt: 1_700_000_000_000,
          updatedAt: 1_700_000_000_500,
        },
        storedAt: Date.now(),
      }),
    );
    (ApiClient as any).prototype.getGames.mockResolvedValue({
      success: true,
      data: [{ id: "g1", name: "Game One", status: "active", wager: 25 }],
    });

    render(<GameLobby />);

    await waitFor(() => {
      expect(screen.getByTestId("lobby-pending-action-chip")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByTestId("lobby-pending-action-chip-dismiss-btn"));
    expect(screen.queryByTestId("lobby-pending-action-chip")).not.toBeInTheDocument();
  });
});
