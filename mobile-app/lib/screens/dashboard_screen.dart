import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../main.dart';
import '../providers/bot_provider.dart';

class DashboardScreen extends StatefulWidget {
  const DashboardScreen({super.key});

  @override
  State<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends State<DashboardScreen> {
  @override
  void initState() {
    super.initState();
    Provider.of<BotProvider>(context, listen: false).fetchStatus();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Arbitrage Bot'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () {
              Provider.of<BotProvider>(context, listen: false).fetchStatus();
            },
          ),
        ],
      ),
      body: Consumer<BotProvider>(
        builder: (context, provider, child) {
          final status = provider.status;

          return SingleChildScrollView(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _buildStatusIndicator(status.isRunning),
                const SizedBox(height: 16),
                _buildControlButtons(provider, status.isRunning),
                const SizedBox(height: 20),
                _buildStatusGrid(status),
                const SizedBox(height: 20),
                _buildRecentTrades(provider.trades),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildStatusIndicator(bool isRunning) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      decoration: BoxDecoration(
        color: isRunning
            ? BalancerTheme.accentGreen.withOpacity(0.1)
            : BalancerTheme.accentRed.withOpacity(0.1),
        borderRadius: BorderRadius.circular(10),
        border: Border.all(
          color: isRunning ? BalancerTheme.accentGreen : BalancerTheme.accentRed,
        ),
      ),
      child: Row(
        children: [
          Container(
            width: 10,
            height: 10,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: isRunning ? BalancerTheme.accentGreen : BalancerTheme.accentRed,
            ),
          ),
          const SizedBox(width: 10),
          Text(
            isRunning ? 'Bot Running' : 'Bot Stopped',
            style: TextStyle(
              color: isRunning ? BalancerTheme.accentGreen : BalancerTheme.accentRed,
              fontWeight: FontWeight.w600,
              fontSize: 15,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildControlButtons(BotProvider provider, bool isRunning) {
    return Row(
      children: [
        Expanded(
          child: ElevatedButton.icon(
            onPressed: isRunning ? null : () => provider.startBot(),
            icon: const Icon(Icons.play_arrow, size: 20),
            label: const Text('Start'),
            style: ElevatedButton.styleFrom(
              backgroundColor: BalancerTheme.accentGreen,
            ),
          ),
        ),
        const SizedBox(width: 8),
        Expanded(
          child: ElevatedButton.icon(
            onPressed: isRunning ? () => provider.stopBot() : null,
            icon: const Icon(Icons.stop, size: 20),
            label: const Text('Stop'),
            style: ElevatedButton.styleFrom(
              backgroundColor: BalancerTheme.accentRed,
            ),
          ),
        ),
        const SizedBox(width: 8),
        Expanded(
          child: ElevatedButton.icon(
            onPressed: () => provider.executeTrade(),
            icon: const Icon(Icons.swap_horiz, size: 20),
            label: const Text('Trade'),
          ),
        ),
      ],
    );
  }

  Widget _buildStatusGrid(BotStatus status) {
    return GridView.count(
      crossAxisCount: 2,
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      childAspectRatio: 1.6,
      mainAxisSpacing: 8,
      crossAxisSpacing: 8,
      children: [
        _buildStatusCard('Total Trades', status.totalTrades.toString(), null),
        _buildStatusCard('Net Profit', '\$${status.totalProfit.toStringAsFixed(2)}',
            status.totalProfit > 0 ? BalancerTheme.accentGreen : BalancerTheme.accentRed),
        _buildStatusCard('Gas Spent', '\$${status.gasSpent.toStringAsFixed(4)}', null),
        _buildStatusCard('Balance', '\$${status.balance.toStringAsFixed(2)}', null),
        _buildStatusCard('Slippage', '${status.avgSlippageBps.toStringAsFixed(0)} bps', null),
        _buildStatusCard('Impact', '${status.avgPriceImpactBps.toStringAsFixed(0)} bps',
            status.avgPriceImpactBps > 50 ? BalancerTheme.accentYellow : null),
        _buildStatusCard('Network', status.network, null),
        _buildStatusCard('Direct', status.useDirectPoolCalls ? 'ON' : 'OFF',
            status.useDirectPoolCalls ? BalancerTheme.accentGreen : BalancerTheme.accentRed),
      ],
    );
  }

  Widget _buildStatusCard(String title, String value, Color? valueColor) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(
              title,
              style: const TextStyle(
                color: BalancerTheme.textMuted,
                fontSize: 11,
                fontWeight: FontWeight.w600,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              value,
              style: TextStyle(
                color: valueColor ?? BalancerTheme.textPrimary,
                fontSize: 18,
                fontWeight: FontWeight.w700,
              ),
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildRecentTrades(List<TradeLog> trades) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'Recent Trades',
          style: TextStyle(
            fontSize: 16,
            fontWeight: FontWeight.w600,
            color: BalancerTheme.textSecondary,
          ),
        ),
        const SizedBox(height: 8),
        if (trades.isEmpty)
          Card(
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Center(
                child: Text(
                  'No trades yet',
                  style: TextStyle(color: BalancerTheme.textMuted),
                ),
              ),
            ),
          )
        else
          ...trades.take(5).map((trade) => _buildTradeCard(trade)),
      ],
    );
  }

  Widget _buildTradeCard(TradeLog trade) {
    final isProfit = trade.profit > 0;
    return Card(
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Row(
          children: [
            Container(
              width: 40,
              height: 40,
              decoration: BoxDecoration(
                color: isProfit
                    ? BalancerTheme.accentGreen.withOpacity(0.1)
                    : BalancerTheme.accentRed.withOpacity(0.1),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(
                isProfit ? Icons.trending_up : Icons.trending_down,
                color: isProfit ? BalancerTheme.accentGreen : BalancerTheme.accentRed,
                size: 20,
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    trade.tokenPair,
                    style: const TextStyle(
                      fontWeight: FontWeight.w600,
                      fontSize: 14,
                      color: BalancerTheme.textPrimary,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    '${trade.dexFrom} → ${trade.dexTo}',
                    style: const TextStyle(
                      fontSize: 12,
                      color: BalancerTheme.textMuted,
                    ),
                  ),
                ],
              ),
            ),
            Column(
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [
                Text(
                  '${isProfit ? '+' : ''}\$${trade.profit.toStringAsFixed(4)}',
                  style: TextStyle(
                    color: isProfit ? BalancerTheme.accentGreen : BalancerTheme.accentRed,
                    fontWeight: FontWeight.w700,
                    fontSize: 14,
                  ),
                ),
                const SizedBox(height: 2),
                Text(
                  'Gas: \$${trade.gasCost.toStringAsFixed(4)}',
                  style: const TextStyle(
                    fontSize: 11,
                    color: BalancerTheme.textMuted,
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}