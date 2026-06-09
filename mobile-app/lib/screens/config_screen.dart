import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../main.dart';
import '../providers/bot_provider.dart';

class ConfigScreen extends StatefulWidget {
  const ConfigScreen({super.key});

  @override
  State<ConfigScreen> createState() => _ConfigScreenState();
}

class _ConfigScreenState extends State<ConfigScreen> {
  final _formKey = GlobalKey<FormState>();
  final _rpcController = TextEditingController(text: 'https://polygon-rpc.com');
  final _privateKeyController = TextEditingController();
  final _contractAddressController = TextEditingController();
  final _borrowAmountController = TextEditingController(text: '10000');
  final _minProfitController = TextEditingController(text: '5');
  final _maxGasController = TextEditingController(text: '100');
  final _scanIntervalController = TextEditingController(text: '1000');
  final _maxSlippageController = TextEditingController(text: '50');
  final _maxImpactController = TextEditingController(text: '1.0');
  final _minBorrowController = TextEditingController(text: '100');
  final _maxBorrowController = TextEditingController(text: '100000');
  final _borrowStepController = TextEditingController(text: '100');

  bool _autoTrade = false;
  bool _simulateBeforeSend = true;
  bool _useDirectPoolCalls = true;
  bool _reverseRoute = false;
  int _startHour = 0;
  int _endHour = 23;

  @override
  void dispose() {
    _rpcController.dispose();
    _privateKeyController.dispose();
    _contractAddressController.dispose();
    _borrowAmountController.dispose();
    _minProfitController.dispose();
    _maxGasController.dispose();
    _scanIntervalController.dispose();
    _maxSlippageController.dispose();
    _maxImpactController.dispose();
    _minBorrowController.dispose();
    _maxBorrowController.dispose();
    _borrowStepController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Configuration'),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Form(
          key: _formKey,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              _buildSection('Network Settings', [
                _buildTextField('RPC URL', _rpcController),
                _buildTextField('Private Key', _privateKeyController, obscure: true),
                _buildTextField('Contract Address', _contractAddressController),
              ]),
              const SizedBox(height: 12),
              _buildSection('Trading Settings', [
                _buildTextField('Default Borrow Amount (USDC)', _borrowAmountController, isNumber: true),
                Row(
                  children: [
                    Expanded(
                      child: _buildTextField('Min Borrow', _minBorrowController, isNumber: true),
                    ),
                    const SizedBox(width: 8),
                    Expanded(
                      child: _buildTextField('Max Borrow', _maxBorrowController, isNumber: true),
                    ),
                    const SizedBox(width: 8),
                    Expanded(
                      child: _buildTextField('Step', _borrowStepController, isNumber: true),
                    ),
                  ],
                ),
                _buildTextField('Minimum Profit (USD)', _minProfitController, isNumber: true),
                _buildTextField('Max Gas Price (Gwei)', _maxGasController, isNumber: true),
                _buildTextField('Scan Interval (ms)', _scanIntervalController, isNumber: true),
              ]),
              const SizedBox(height: 12),
              _buildSection('Slippage & Price Impact', [
                _buildTextField('Max Slippage (bps)', _maxSlippageController, isNumber: true),
                _buildTextField('Max Price Impact (%)', _maxImpactController, isNumber: true),
              ]),
              const SizedBox(height: 12),
              _buildSection('Routing', [
                _buildSwitch('Direct Pool Calls', _useDirectPoolCalls, (v) => setState(() => _useDirectPoolCalls = v)),
                _buildSwitch('Reverse Route', _reverseRoute, (v) => setState(() => _reverseRoute = v)),
              ]),
              const SizedBox(height: 12),
              _buildSection('Simulation', [
                _buildSwitch('Simulate Before Send', _simulateBeforeSend, (v) => setState(() => _simulateBeforeSend = v)),
              ]),
              const SizedBox(height: 12),
              _buildSection('Schedule', [
                _buildSwitch('Auto-trade', _autoTrade, (v) => setState(() => _autoTrade = v)),
                Row(
                  children: [
                    Expanded(
                      child: DropdownButtonFormField<int>(
                        value: _startHour,
                        decoration: const InputDecoration(
                          labelText: 'Start Hour (UTC)',
                        ),
                        items: List.generate(24, (i) => DropdownMenuItem(
                          value: i,
                          child: Text('$i:00'),
                        )),
                        onChanged: (value) => setState(() => _startHour = value ?? 0),
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: DropdownButtonFormField<int>(
                        value: _endHour,
                        decoration: const InputDecoration(
                          labelText: 'End Hour (UTC)',
                        ),
                        items: List.generate(24, (i) => DropdownMenuItem(
                          value: i,
                          child: Text('$i:00'),
                        )),
                        onChanged: (value) => setState(() => _endHour = value ?? 23),
                      ),
                    ),
                  ],
                ),
              ]),
              const SizedBox(height: 20),
              SizedBox(
                width: double.infinity,
                child: ElevatedButton(
                  onPressed: _saveConfig,
                  child: const Text('Save Configuration'),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildSection(String title, List<Widget> children) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(
                fontSize: 15,
                fontWeight: FontWeight.w600,
                color: BalancerTheme.textSecondary,
              ),
            ),
            const SizedBox(height: 12),
            ...children,
          ],
        ),
      ),
    );
  }

  Widget _buildTextField(String label, TextEditingController controller, {
    bool obscure = false,
    bool isNumber = false,
  }) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: TextFormField(
        controller: controller,
        obscureText: obscure,
        keyboardType: isNumber ? TextInputType.number : TextInputType.text,
        decoration: InputDecoration(
          labelText: label,
        ),
      ),
    );
  }

  Widget _buildSwitch(String label, bool value, Function(bool) onChanged) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            label,
            style: const TextStyle(
              fontSize: 14,
              color: BalancerTheme.textSecondary,
            ),
          ),
          Switch(
            value: value,
            onChanged: onChanged,
          ),
        ],
      ),
    );
  }

  void _saveConfig() {
    if (_formKey.currentState!.validate()) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: const Text('Configuration saved'),
          backgroundColor: BalancerTheme.accentTeal,
          behavior: SnackBarBehavior.floating,
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
        ),
      );
    }
  }
}