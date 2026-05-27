/**
 * Components v1 - Public API
 *
 * Re-exports all v1 components for clean imports.
 *
 * @example
 * ```tsx
 * import { EmptyStateBlock, WalletStatusCard } from '@/components/v1';
 * ```
 */

export {
  AccountSwitcher,
  default as AccountSwitcherDefault,
} from "./AccountSwitcher";
export type {
  AccountSwitcherProps,
  RecentAccount,
} from "./AccountSwitcher.types";

export {
  EmptyStateBlock,
  default as EmptyStateBlockDefault,
} from "./EmptyStateBlock";
export type {
  EmptyStateBlockProps,
  EmptyStateAction,
  EmptyStateVariant,
  ActionVariant,
} from "./EmptyStateBlock.types";

export { default as ErrorNotice } from "./ErrorNotice";
export type { ErrorNoticeProps } from "./ErrorNotice";

export {
  FormErrorSummary,
  default as FormErrorSummaryDefault,
} from "./FormErrorSummary";
export type { FormErrorSummaryProps, FormFieldError } from "./FormErrorSummary";

export { ActionToolbar } from "./ActionToolbar";
export type {
  ActionToolbarProps,
  ToolbarAction,
  ToolbarActionIntent,
} from "./ActionToolbar";

export {
  ContractEventFeed,
  default as ContractEventFeedDefault,
} from "./ContractEventFeed";
export type { ContractEventFeedProps } from "./ContractEventFeed";

export { PaginatedListController } from "./PaginatedListController";
export type { PaginatedListControllerProps } from "./PaginatedListController";

export {
  WalletStatusCard,
  default as WalletStatusCardDefault,
} from "./WalletStatusCard";
export type {
  WalletStatusCardProps,
  WalletStatusCardCallbacks,
  WalletBadgeVariant,
  WalletStatus,
  WalletCapabilities,
  WalletStatusError,
  WalletDiagnosticItem,
} from "./WalletStatusCard.types";

export { StatusPill, default as StatusPillDefault } from "./StatusPill";
export type {
  StatusPillProps,
  StatusPillTone,
  StatusPillSize,
} from "./StatusPill";

export { AsyncStateBoundary } from "./AsyncStateBoundary";
export type { AsyncStateBoundaryProps } from "./AsyncStateBoundary";

export { ContractActionButton } from "./ContractActionButton";
export type { ContractActionButtonProps } from "./ContractActionButton";

export {
  SessionTimeoutModal,
  default as SessionTimeoutModalDefault,
} from "./SessionTimeoutModal";
export type { SessionTimeoutModalProps } from "./SessionTimeoutModal";

export {
  SegmentedControl,
  default as SegmentedControlDefault,
} from "./SegmentedControl";
export type {
  SegmentedControlOption,
  SegmentedControlProps,
} from "./SegmentedControl";

export {
  NotificationCenter,
  default as NotificationCenterDefault,
} from "./NotificationCenter";

export {
  ResumeTaskBanner,
  default as ResumeTaskBannerDefault,
} from "./ResumeTaskBanner";
export type { ResumeTaskBannerProps } from "./ResumeTaskBanner";

export {
  PendingActionResumeChip,
  default as PendingActionResumeChipDefault,
} from "./PendingActionResumeChip";
export type { PendingActionResumeChipProps } from "./PendingActionResumeChip";

export {
  NotificationPreferencesPanel,
  default as NotificationPreferencesPanelDefault,
} from "./NotificationPreferencesPanel";
export type { NotificationPreferencesPanelProps } from "./NotificationPreferencesPanel";

export {
  SkeletonBase,
  SkeletonCard,
  SkeletonRow,
  SkeletonList,
  SkeletonPreset,
  LoadingState,
  PageSkeletonOrchestrator,
} from "./LoadingSkeletonSet";
export type {
  SkeletonBaseProps,
  SkeletonCardProps,
  SkeletonRowProps,
  SkeletonListProps,
  SkeletonPresetProps,
  LoadingStateProps,
  PageSkeletonOrchestratorProps,
  PageSkeletonSurface,
  PageSkeletonSurfaceStatus,
} from "./LoadingSkeletonSet";

export {
  SKELETON_PRESETS,
  skBaseColor,
  skBaseColorDark,
  skBorderColor,
  skBorderColorDark,
  skRadiusSm,
  skRadiusMd,
  skRadiusLg,
  skRadiusCircle,
  skGapSm,
  skGapMd,
  skGapLg,
  skPadding,
  skPulseDuration,
  skPulseEasing,
  skHeightTextSm,
  skHeightTextMd,
  skHeightTextLg,
  skHeightHeading,
  skHeightThumbnail,
  skHeightDetailBanner,
  skSizeAvatarSm,
  skSizeAvatarMd,
  skSizeAvatarLg,
} from "./skeleton.tokens";
export type { SkeletonShape, SkeletonPresetType } from "./skeleton.tokens";

// Quest Components
export { QuestCard, default as QuestCardDefault } from "./QuestCard";
export type { QuestCardProps } from "../../types/v1/quest";

export {
  QuestProgressBar,
  default as QuestProgressBarDefault,
} from "./QuestProgressBar";
export type { QuestProgressBarProps } from "../../types/v1/quest";

export {
  QuestProgressRing,
  default as QuestProgressRingDefault,
} from "./QuestProgressRing";
export type { QuestProgressRingProps } from "../../types/v1/quest";

export {
  QuestWorkspaceHeader,
  default as QuestWorkspaceHeaderDefault,
} from "./QuestWorkspaceHeader";
export type { QuestWorkspaceHeaderProps } from "../../types/v1/quest";
export { Timeline, default as TimelineDefault } from "./Timeline";
export type {
  TimelineProps,
  TimelineItemData,
  TimelineItemStatus,
} from "./Timeline";

// New Components - Issue #637, #636, #635, #634
export {
  QuickPivotLinks,
  default as QuickPivotLinksDefault,
} from "./QuickPivotLinks";
export type {
  QuickPivotLinksProps,
  PivotLink,
} from "./QuickPivotLinks";

export {
  AnalyticsRangeSwitcher,
  default as AnalyticsRangeSwitcherDefault,
} from "./AnalyticsRangeSwitcher";
export type {
  AnalyticsRangeSwitcherProps,
  TimeRange,
} from "./AnalyticsRangeSwitcher";

export {
  DashboardEmptyPanelShell,
  default as DashboardEmptyPanelShellDefault,
} from "./DashboardEmptyPanelShell";
export type { DashboardEmptyPanelShellProps } from "./DashboardEmptyPanelShell";

export {
  CampaignRewardsSpotlightCard,
  default as CampaignRewardsSpotlightCardDefault,
} from "./CampaignRewardsSpotlightCard";
export type {
  CampaignRewardsSpotlightCardProps,
} from "./CampaignRewardsSpotlightCard";

export {
  PinnedWalletActionTray,
  default as PinnedWalletActionTrayDefault,
} from "./PinnedWalletActionTray";
export type {
  PinnedWalletActionTrayProps,
  WalletActionItem,
} from "./PinnedWalletActionTray";

export {
  AuditSnapshotCard,
  default as AuditSnapshotCardDefault,
} from "./AuditSnapshotCard";
export type {
  AuditSnapshotCardProps,
  AuditSnapshot,
} from "./AuditSnapshotCard";

export {
  QueueHealthWidget,
  default as QueueHealthWidgetDefault,
} from "./QueueHealthWidget";
export type {
  QueueHealthWidgetProps,
  QueueMetrics,
} from "./QueueHealthWidget";

export {
  StickyActionsFooter,
  default as StickyActionsFooterDefault,
} from "./StickyActionsFooter";
export type { StickyActionsFooterProps } from "./StickyActionsFooter";

export {
  WalletBalanceDeltaCards,
  default as WalletBalanceDeltaCardsDefault,
} from "./WalletBalanceDeltaCards";
export type {
  WalletBalanceDeltaCardsProps,
  WalletComparisonBalance,
} from "./WalletBalanceDeltaCards";

// Issues #621–#624
export {
  ReorderableList,
  default as ReorderableListDefault,
} from "./ReorderableList";
export type {
  ReorderableListProps,
  ReorderableListItem,
} from "./ReorderableList";

export { MetricCard, default as MetricCardDefault } from "./MetricCard";
export type {
  MetricCardProps,
  MetricCardStatus,
  MetricTrend,
} from "./MetricCard";

export {
  EventDigestPanel,
  default as EventDigestPanelDefault,
} from "./EventDigestPanel";
export type {
  EventDigestPanelProps,
  DigestEvent,
  DigestEventSeverity,
} from "./EventDigestPanel";

export {
  InlineStatDelta,
  default as InlineStatDeltaDefault,
} from "./InlineStatDelta";
export type { InlineStatDeltaProps } from "./InlineStatDelta";

export {
  ReviewSubmitSheet,
  default as ReviewSubmitSheetDefault,
} from "./ReviewSubmitSheet";
export type {
  ReviewSubmitSheetProps,
  ReviewField,
  RiskLevel,
} from "./ReviewSubmitSheet";

// Issue #705
export {
  ContractHealthRibbon,
  default as ContractHealthRibbonDefault,
} from "./ContractHealthRibbon";
export type {
  ContractHealthRibbonProps,
  ContractHealthStatus,
  ContractHealthRibbonVariant,
} from "./ContractHealthRibbon";
