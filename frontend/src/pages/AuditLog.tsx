/**
 * AuditLog Page
 * 
 * Demonstrates integration of AuditSnapshotCard in narrow detail layouts
 */

import React, { useState } from 'react';
import { AuditSnapshotCard, type AuditSnapshot } from '../components/v1/AuditSnapshotCard';
import { AnalyticsRangeSwitcher, type TimeRange } from '../components/v1/AnalyticsRangeSwitcher';

const AuditLog: React.FC = () => {
  const [selectedRange, setSelectedRange] = useState<string>('24h');
  const [selectedVariant, setSelectedVariant] = useState<'minimal' | 'standard' | 'detailed'>('standard');

  const timeRanges: TimeRange[] = [
    { id: '1h', label: '1 Hour', shortLabel: '1H', value: '1h' },
    { id: '24h', label: '24 Hours', shortLabel: '24H', value: '24h' },
    { id: '7d', label: '7 Days', shortLabel: '7D', value: '7d' },
    { id: '30d', label: '30 Days', shortLabel: '30D', value: '30d' },
  ];

  // Mock audit data
  const generateAuditData = (rangeId: string): AuditSnapshot[] => {
    const baseTime = new Date();
    const getTimeOffset = (index: number) => {
      const multipliers = { '1h': 5, '24h': 120, '7d': 8640, '30d': 36000 };
      return multipliers[rangeId as keyof typeof multipliers] * index * 1000;
    };

    return [
      {
        id: 'audit-1',
        timestamp: new Date(baseTime.getTime() - getTimeOffset(1)).toISOString(),
        action: 'User Authentication',
        actor: 'alice.stellar@example.com',
        status: 'success',
        target: 'user:alice_123',
        details: {
          method: 'stellar_wallet',
          walletAddress: 'GCKFBEIYTKP...',
          sessionId: 'sess_abc123',
        },
        metadata: {
          ip: '192.168.1.100',
          userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)',
          location: 'San Francisco, CA',
          duration: 1250,
        },
      },
      {
        id: 'audit-2',
        timestamp: new Date(baseTime.getTime() - getTimeOffset(2)).toISOString(),
        action: 'Game Contract Deployment',
        actor: 'system@stellarcade.com',
        status: 'success',
        target: 'contract:coin_flip_v2',
        details: {
          contractAddress: 'CCJZ5DGASBWQ...',
          gasUsed: 45000,
          deploymentHash: '0x1234567890abcdef',
        },
        metadata: {
          ip: '10.0.0.1',
          duration: 8500,
        },
      },
      {
        id: 'audit-3',
        timestamp: new Date(baseTime.getTime() - getTimeOffset(3)).toISOString(),
        action: 'Prize Pool Withdrawal',
        actor: 'bob.gamer@example.com',
        status: 'warning',
        target: 'pool:weekly_tournament',
        details: {
          amount: '150.75 XLM',
          transactionId: 'tx_def456',
          reason: 'tournament_win',
        },
        metadata: {
          ip: '203.0.113.45',
          userAgent: 'StellarCade Mobile App v1.2.3',
          location: 'Tokyo, Japan',
          duration: 3200,
        },
      },
      {
        id: 'audit-4',
        timestamp: new Date(baseTime.getTime() - getTimeOffset(4)).toISOString(),
        action: 'Failed Login Attempt',
        actor: 'unknown@suspicious.com',
        status: 'error',
        target: 'auth:login_endpoint',
        details: {
          reason: 'invalid_signature',
          attemptCount: 5,
          blocked: true,
        },
        metadata: {
          ip: '198.51.100.42',
          userAgent: 'curl/7.68.0',
          location: 'Unknown',
          duration: 150,
        },
      },
      {
        id: 'audit-5',
        timestamp: new Date(baseTime.getTime() - getTimeOffset(5)).toISOString(),
        action: 'Admin Panel Access',
        actor: 'admin@stellarcade.com',
        status: 'success',
        target: 'admin:dashboard',
        details: {
          section: 'user_management',
          permissions: ['read', 'write', 'delete'],
        },
        metadata: {
          ip: '172.16.0.10',
          userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)',
          location: 'New York, NY',
          duration: 450,
        },
      },
      {
        id: 'audit-6',
        timestamp: new Date(baseTime.getTime() - getTimeOffset(6)).toISOString(),
        action: 'Smart Contract Interaction',
        actor: 'charlie.dev@example.com',
        status: 'pending',
        target: 'contract:random_generator',
        details: {
          function: 'generate_random_number',
          parameters: { seed: 'user_input', nonce: 12345 },
        },
        metadata: {
          ip: '10.0.0.25',
          duration: 2100,
        },
      },
    ];
  };

  const auditData = generateAuditData(selectedRange);

  const handleRangeChange = (rangeId: string) => {
    setSelectedRange(rangeId);
  };

  const handleAuditClick = (audit: AuditSnapshot) => {
    console.log('Audit clicked:', audit);
    // In a real app, this might open a detailed view or modal
  };

  return (
    <main style={{ padding: '2rem', maxWidth: '1200px', margin: '0 auto' }} aria-labelledby="audit-log-heading">
      <header style={{ marginBottom: '2rem' }}>
        <h1 id="audit-log-heading">Audit Log</h1>
        <p style={{ color: '#a8b5c8' }}>
          Monitor system activities, user actions, and security events
        </p>
      </header>

      <section aria-labelledby="audit-recent-activity-heading" style={{ 
        display: 'flex', 
        alignItems: 'center', 
        justifyContent: 'space-between', 
        marginBottom: '2rem',
        flexWrap: 'wrap',
        gap: '1rem'
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
          <h2 id="audit-recent-activity-heading" style={{ margin: 0 }}>Recent Activity</h2>
          <select 
            value={selectedVariant}
            onChange={(e) => setSelectedVariant(e.target.value as any)}
            style={{
              padding: '0.5rem',
              borderRadius: '0.375rem',
              border: '1px solid #333',
              background: 'rgba(255, 255, 255, 0.08)',
              color: '#f5f7fb',
              fontSize: '0.875rem'
            }}
          >
            <option value="minimal">Minimal</option>
            <option value="standard">Standard</option>
            <option value="detailed">Detailed</option>
          </select>
        </div>
        
        <AnalyticsRangeSwitcher
          ranges={timeRanges}
          selectedId={selectedRange}
          onChange={handleRangeChange}
          size="compact"
          testId="audit-log-range-switcher"
        />
      </section>

      <section aria-label="Audit activity list" style={{ 
        display: 'grid', 
        gap: '1rem',
        gridTemplateColumns: selectedVariant === 'minimal' 
          ? 'repeat(auto-fit, minmax(300px, 1fr))'
          : '1fr'
      }}>
        {auditData.map((audit) => (
          <AuditSnapshotCard
            key={audit.id}
            audit={audit}
            variant={selectedVariant}
            expandable={selectedVariant !== 'minimal'}
            onClick={handleAuditClick}
            testId={`audit-card-${audit.id}`}
          />
        ))}
      </section>

      {auditData.length === 0 && (
        <div style={{ 
          padding: '3rem', 
          textAlign: 'center', 
          color: '#a8b5c8',
          border: '1px dashed #333',
          borderRadius: '0.75rem'
        }}>
          <h3>No audit events found</h3>
          <p>No activities recorded for the selected time period.</p>
        </div>
      )}

      <section aria-label="Audit card variant guidance" style={{ 
        marginTop: '3rem',
        padding: '1.5rem',
        border: '1px solid #333',
        borderRadius: '0.75rem',
        background: 'rgba(255, 255, 255, 0.02)'
      }}>
        <h3 style={{ marginBottom: '1rem' }}>Audit Card Variants</h3>
        <div style={{ display: 'grid', gap: '1rem', marginBottom: '1rem' }}>
          <div>
            <strong>Minimal:</strong> Compact layout for narrow sidebars or mobile views. 
            Shows essential information with truncated text.
          </div>
          <div>
            <strong>Standard:</strong> Balanced layout showing action, actor, status, and target. 
            Expandable to show full details.
          </div>
          <div>
            <strong>Detailed:</strong> Full layout with all information visible. 
            Best for main content areas with ample space.
          </div>
        </div>
        
        <p style={{ color: '#a8b5c8', fontSize: '0.875rem' }}>
          The AuditSnapshotCard component automatically handles responsive behavior, 
          loading states, and accessibility requirements. It's designed to work well 
          in narrow detail layouts while maintaining readability and functionality.
        </p>
      </section>
    </main>
  );
};

export default AuditLog;
