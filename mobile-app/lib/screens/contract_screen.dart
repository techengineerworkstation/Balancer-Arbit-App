import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../main.dart';
import '../providers/bot_provider.dart';

class ContractScreen extends StatelessWidget {
  const ContractScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Contract'),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _buildDeploymentGuide(context),
            const SizedBox(height: 16),
            _buildGasEstimates(context),
            const SizedBox(height: 16),
            _buildContractActions(context),
          ],
        ),
      ),
    );
  }

  Widget _buildDeploymentGuide(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Deployment Guide',
              style: TextStyle(
                fontSize: 15,
                fontWeight: FontWeight.w600,
                color: BalancerTheme.textSecondary,
              ),
            ),
            const SizedBox(height: 12),
            _buildGuideStep(
              '1. Install Foundry',
              'curl -L https://foundry.paradigm.xyz | bash && foundryup',
            ),
            _buildGuideStep(
              '2. Deploy Contract',
              'PRIVATE_KEY=0x... forge script script/Deploy.s.sol --rpc-url https://polygon-rpc.com --broadcast',
            ),
            _buildGuideStep(
              '3. Verify Contract',
              'forge verify-contract <addr> BalancerFlashLoanArbitrage --chain-id 137',
            ),
            _buildGuideStep(
              '4. Fund Contract',
              'Send MATIC for gas, then execute flash loans via the bot',
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildGuideStep(String title, String command) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            title,
            style: const TextStyle(
              fontWeight: FontWeight.w600,
              color: BalancerTheme.accentTeal,
              fontSize: 13,
            ),
          ),
          const SizedBox(height: 4),
          Container(
            width: double.infinity,
            padding: const EdgeInsets.all(10),
            decoration: BoxDecoration(
              color: BalancerTheme.bgPrimary,
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: BalancerTheme.borderLight),
            ),
            child: Text(
              command,
              style: const TextStyle(
                fontFamily: 'monospace',
                fontSize: 11,
                color: BalancerTheme.accentTealDark,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildGasEstimates(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Gas Estimates (Polygon)',
              style: TextStyle(
                fontSize: 15,
                fontWeight: FontWeight.w600,
                color: BalancerTheme.textSecondary,
              ),
            ),
            const SizedBox(height: 12),
            _buildGasRow('Deployment', '~3M gas', '\$0.10-\$0.30'),
            _buildGasRow('Flash Loan + Swap', '~500K gas', '\$0.02-\$0.05'),
            _buildGasRow('Flash Loan + 2 Swaps', '~750K gas', '\$0.03-\$0.08'),
            _buildGasRow('Withdraw Profits', '~100K gas', '\$0.005-\$0.01'),
          ],
        ),
      ),
    );
  }

  Widget _buildGasRow(String operation, String gas, String cost) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: Row(
        children: [
          Expanded(
            flex: 3,
            child: Text(
              operation,
              style: const TextStyle(
                fontSize: 13,
                color: BalancerTheme.textPrimary,
              ),
            ),
          ),
          Expanded(
            flex: 2,
            child: Text(
              gas,
              style: const TextStyle(
                fontSize: 13,
                color: BalancerTheme.textSecondary,
              ),
            ),
          ),
          Expanded(
            flex: 2,
            child: Text(
              cost,
              style: const TextStyle(
                fontSize: 13,
                fontWeight: FontWeight.w600,
                color: BalancerTheme.accentTeal,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildContractActions(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Actions',
              style: TextStyle(
                fontSize: 15,
                fontWeight: FontWeight.w600,
                color: BalancerTheme.textSecondary,
              ),
            ),
            const SizedBox(height: 12),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton.icon(
                onPressed: () {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: const Text('Checking balance...'),
                      backgroundColor: BalancerTheme.accentTeal,
                      behavior: SnackBarBehavior.floating,
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                    ),
                  );
                },
                icon: const Icon(Icons.account_balance_wallet, size: 18),
                label: const Text('Check Balance'),
              ),
            ),
            const SizedBox(height: 8),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton.icon(
                onPressed: () {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: const Text('Withdrawing profits...'),
                      backgroundColor: BalancerTheme.accentYellow,
                      behavior: SnackBarBehavior.floating,
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                    ),
                  );
                },
                icon: const Icon(Icons.arrow_downward, size: 18),
                label: const Text('Withdraw Profits'),
                style: ElevatedButton.styleFrom(
                  backgroundColor: BalancerTheme.accentYellow,
                  foregroundColor: BalancerTheme.textPrimary,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}