import React from "react";
import "./StickyActionsFooter.css";

export interface StickyActionsFooterProps {
  children: React.ReactNode;
  className?: string;
  testId?: string;
}

export const StickyActionsFooter: React.FC<StickyActionsFooterProps> = ({
  children,
  className = "",
  testId = "sticky-actions-footer",
}) => {
  return (
    <div
      className={`sticky-actions-footer ${className}`.trim()}
      data-testid={testId}
      role="region"
      aria-label="Page actions"
    >
      <div className="sticky-actions-footer__inner">{children}</div>
    </div>
  );
};

export default StickyActionsFooter;
