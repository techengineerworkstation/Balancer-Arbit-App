import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class BotStatus {
  final bool isRunning;
  final int totalTrades;
  final double totalProfit;
  final String? lastTradeTime;
  final double gasSpent;
  final double balance;
  final String network;

  BotStatus({
    required this.isRunning,
    required this.totalTrades,
    required this.totalProfit,
    this.lastTradeTime,
    required this.gasSpent,
    required this.balance,
    required this.network,
  });

  factory BotStatus.fromJson(Map<String, dynamic> json) {
    return BotStatus(
      isRunning: json['is_running'] ?? false,
      totalTrades: json['total_trades'] ?? 0,
      totalProfit: (json['total_profit'] ?? 0).toDouble(),
      lastTradeTime: json['last_trade_time'],
      gasSpent: (json['gas_spent'] ?? 0).toDouble(),
      balance: (json['balance'] ?? 0).toDouble(),
      network: json['network'] ?? 'Arbitrum One',
    );
  }
}

class TradeLog {
  final int id;
  final String timestamp;
  final String tokenPair;
  final String dexFrom;
  final String dexTo;
  final double amount;
  final double profit;
  final double gasCost;
  final String status;

  TradeLog({
    required this.id,
    required this.timestamp,
    required this.tokenPair,
    required this.dexFrom,
    required this.dexTo,
    required this.amount,
    required this.profit,
    required this.gasCost,
    required this.status,
  });

  factory TradeLog.fromJson(Map<String, dynamic> json) {
    return TradeLog(
      id: json['id'] ?? 0,
      timestamp: json['timestamp'] ?? '',
      tokenPair: json['token_pair'] ?? '',
      dexFrom: json['dex_from'] ?? '',
      dexTo: json['dex_to'] ?? '',
      amount: (json['amount'] ?? 0).toDouble(),
      profit: (json['profit'] ?? 0).toDouble(),
      gasCost: (json['gas_cost'] ?? 0).toDouble(),
      status: json['status'] ?? '',
    );
  }
}

class BotProvider extends ChangeNotifier {
  String _baseUrl = 'http://localhost:8080';
  BotStatus _status = BotStatus(
    isRunning: false,
    totalTrades: 0,
    totalProfit: 0,
    gasSpent: 0,
    balance: 0,
    network: 'Arbitrum One',
  );
  List<TradeLog> _trades = [];
  String? _error;

  BotStatus get status => _status;
  List<TradeLog> get trades => _trades;
  String? get error => _error;

  void setBaseUrl(String url) {
    _baseUrl = url;
    notifyListeners();
  }

  Future<void> fetchStatus() async {
    try {
      final response = await http.get(Uri.parse('$_baseUrl/api/status'));
      if (response.statusCode == 200) {
        _status = BotStatus.fromJson(json.decode(response.body));
        _error = null;
        notifyListeners();
      }
    } catch (e) {
      _error = e.toString();
      notifyListeners();
    }
  }

  Future<void> fetchTrades() async {
    try {
      final response = await http.get(Uri.parse('$_baseUrl/api/trades'));
      if (response.statusCode == 200) {
        final List<dynamic> data = json.decode(response.body);
        _trades = data.map((t) => TradeLog.fromJson(t)).toList();
        _error = null;
        notifyListeners();
      }
    } catch (e) {
      _error = e.toString();
      notifyListeners();
    }
  }

  Future<void> startBot() async {
    try {
      final response = await http.post(Uri.parse('$_baseUrl/api/bot/start'));
      if (response.statusCode == 200) {
        await fetchStatus();
      }
    } catch (e) {
      _error = e.toString();
      notifyListeners();
    }
  }

  Future<void> stopBot() async {
    try {
      final response = await http.post(Uri.parse('$_baseUrl/api/bot/stop'));
      if (response.statusCode == 200) {
        await fetchStatus();
      }
    } catch (e) {
      _error = e.toString();
      notifyListeners();
    }
  }

  Future<void> executeTrade() async {
    try {
      final response = await http.post(Uri.parse('$_baseUrl/api/bot/execute'));
      if (response.statusCode == 200) {
        await fetchStatus();
        await fetchTrades();
      }
    } catch (e) {
      _error = e.toString();
      notifyListeners();
    }
  }
}