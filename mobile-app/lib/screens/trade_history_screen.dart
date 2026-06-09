import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../main.dart';
import '../providers/bot_provider.dart';

class TradeHistoryScreen extends StatefulWidget {
  const TradeHistoryScreen({super.key});

  @override
  State<TradeHistoryScreen> createState() => _TradeHistoryScreenState();
}

class _TradeHistoryScreenState extends State<TradeHistoryScreen> {
  @override
  void initState() {
    super.initState();
    Provider.of<BotProvider>(context, listen: false).fetchTrades();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Trade History'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () {
              Provider.of<BotProvider>(context, listen: false).fetchTrades();
            },
          ),
        ],
      ),
      body: Consumer<BotProvider>(
        builder: (context, provider, child) {
          final trades = provider.trades;

          if (trades.isEmpty) {
            return Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(Icons.history, size: 64, color: BalancerTheme.textMuted.withOpacity(0.3)),
                  const SizedBox(height: 16),
                  const Text(
                    'No trades yet',
                    style: TextStyle(color: BalancerTheme.textMuted, fontSize: 16),
                  ),
                ],
              ),
            );
          }

          return ListView.builder(
            padding: const EdgeInsets.all(12),
            itemCount: trades.length,
            itemBuilder: (context, index) {
              final trade = trades[index];
              final isProfit = trade.profit > 0;

              return Card(
                margin: const EdgeInsets.only(bottom: 8),
                child: Padding(
                  padding: const EdgeInsets.all(12),
                  child: Row(
                    children: [
                      Container(
                        width: 44,
                        height: 44,
                        decoration: BoxDecoration(
                          color: isProfit
                              ? BalancerTheme.accentGreen.withOpacity(0.1)
                              : BalancerTheme.accentRed.withOpacity(0.1),
                          borderRadius: BorderRadius.circular(10),
                        ),
                        child: Icon(
                          isProfit ? Icons.trending_up : Icons.trending_down,
                          color: isProfit ? BalancerTheme.accentGreen : BalancerTheme.accentRed,
                          size: 22,
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
                            const SizedBox(height: 2),
                            Text(
                              trade.timestamp,
                              style: const TextStyle(
                                fontSize: 11,
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
            },
          );
        },
      ),
    );
  }
}