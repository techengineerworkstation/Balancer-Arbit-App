import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:provider/provider.dart';

import 'providers/bot_provider.dart';
import 'screens/dashboard_screen.dart';
import 'screens/config_screen.dart';
import 'screens/trade_history_screen.dart';
import 'screens/contract_screen.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await dotenv.load(fileName: ".env");

  SystemChrome.setSystemUIOverlayStyle(const SystemUiOverlayStyle(
    statusBarColor: Colors.transparent,
    statusBarIconBrightness: Brightness.dark,
  ));

  runApp(const MyApp());
}

class BalancerTheme {
  static const Color bgPrimary = Color(0xFFF5F0E8);
  static const Color bgSecondary = Color(0xFFEFE9DD);
  static const Color bgCard = Color(0xFFFAF7F0);
  static const Color textPrimary = Color(0xFF1A2E35);
  static const Color textSecondary = Color(0xFF4A6670);
  static const Color textMuted = Color(0xFF7A949E);
  static const Color accentTeal = Color(0xFF2A9D8F);
  static const Color accentTealLight = Color(0xFF40B5A6);
  static const Color accentTealDark = Color(0xFF1F7A6F);
  static const Color accentBeige = Color(0xFFD4C5A9);
  static const Color accentGreen = Color(0xFF57A773);
  static const Color accentRed = Color(0xFFC44536);
  static const Color accentYellow = Color(0xFFD4A843);
  static const Color border = Color(0xFFD4C5A9);
  static const Color borderLight = Color(0xFFE8DCC8);
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => BotProvider(),
      child: MaterialApp(
        title: 'Balancer Arbitrage Bot',
        debugShowCheckedModeBanner: false,
        theme: ThemeData(
          brightness: Brightness.light,
          scaffoldBackgroundColor: BalancerTheme.bgPrimary,
          colorScheme: ColorScheme.light(
            primary: BalancerTheme.accentTeal,
            secondary: BalancerTheme.accentTealLight,
            surface: BalancerTheme.bgCard,
            error: BalancerTheme.accentRed,
            onPrimary: Colors.white,
            onSecondary: Colors.white,
            onSurface: BalancerTheme.textPrimary,
          ),
          appBarTheme: const AppBarTheme(
            backgroundColor: BalancerTheme.accentTeal,
            foregroundColor: Colors.white,
            elevation: 2,
            shadowColor: Color(0x332A9D8F),
            titleTextStyle: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.w600,
              color: Colors.white,
              letterSpacing: -0.3,
            ),
          ),
          cardTheme: CardThemeData(
            color: BalancerTheme.bgCard,
            elevation: 1,
            shadowColor: const Color(0x142A9D8F),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(10),
              side: const BorderSide(color: BalancerTheme.borderLight),
            ),
          ),
          elevatedButtonTheme: ElevatedButtonThemeData(
            style: ElevatedButton.styleFrom(
              backgroundColor: BalancerTheme.accentTeal,
              foregroundColor: Colors.white,
              elevation: 2,
              shadowColor: const Color(0x402A9D8F),
              padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 14),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(10),
              ),
              textStyle: const TextStyle(
                fontWeight: FontWeight.w600,
                fontSize: 14,
              ),
            ),
          ),
          inputDecorationTheme: InputDecorationTheme(
            filled: true,
            fillColor: BalancerTheme.bgPrimary,
            border: OutlineInputBorder(
              borderRadius: BorderRadius.circular(8),
              borderSide: const BorderSide(color: BalancerTheme.border),
            ),
            enabledBorder: OutlineInputBorder(
              borderRadius: BorderRadius.circular(8),
              borderSide: const BorderSide(color: BalancerTheme.border),
            ),
            focusedBorder: OutlineInputBorder(
              borderRadius: BorderRadius.circular(8),
              borderSide: const BorderSide(color: BalancerTheme.accentTeal, width: 2),
            ),
            contentPadding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
            labelStyle: const TextStyle(color: BalancerTheme.textSecondary, fontSize: 14),
            hintStyle: const TextStyle(color: BalancerTheme.textMuted, fontSize: 14),
          ),
          switchTheme: SwitchThemeData(
            thumbColor: WidgetStateProperty.resolveWith((states) {
              if (states.contains(WidgetState.selected)) {
                return Colors.white;
              }
              return BalancerTheme.textMuted;
            }),
            trackColor: WidgetStateProperty.resolveWith((states) {
              if (states.contains(WidgetState.selected)) {
                return BalancerTheme.accentTeal;
              }
              return BalancerTheme.border;
            }),
          ),
          navigationBarTheme: NavigationBarThemeData(
            backgroundColor: BalancerTheme.bgCard,
            indicatorColor: const Color(0x1A2A9D8F),
            elevation: 2,
            shadowColor: const Color(0x142A9D8F),
            labelTextStyle: WidgetStateProperty.resolveWith((states) {
              if (states.contains(WidgetState.selected)) {
                return const TextStyle(
                  color: BalancerTheme.accentTeal,
                  fontWeight: FontWeight.w600,
                  fontSize: 12,
                );
              }
              return const TextStyle(
                color: BalancerTheme.textMuted,
                fontSize: 12,
              );
            }),
            iconTheme: WidgetStateProperty.resolveWith((states) {
              if (states.contains(WidgetState.selected)) {
                return const IconThemeData(color: BalancerTheme.accentTeal);
              }
              return const IconThemeData(color: BalancerTheme.textMuted);
            }),
          ),
          dividerTheme: const DividerThemeData(
            color: BalancerTheme.borderLight,
            thickness: 1,
          ),
          textTheme: const TextTheme(
            headlineLarge: TextStyle(color: BalancerTheme.textPrimary, fontWeight: FontWeight.w700),
            headlineMedium: TextStyle(color: BalancerTheme.textPrimary, fontWeight: FontWeight.w600),
            titleLarge: TextStyle(color: BalancerTheme.textPrimary, fontWeight: FontWeight.w600),
            titleMedium: TextStyle(color: BalancerTheme.textSecondary, fontWeight: FontWeight.w600),
            bodyLarge: TextStyle(color: BalancerTheme.textPrimary),
            bodyMedium: TextStyle(color: BalancerTheme.textSecondary),
            bodySmall: TextStyle(color: BalancerTheme.textMuted),
            labelLarge: TextStyle(color: BalancerTheme.accentTeal, fontWeight: FontWeight.w600),
          ),
        ),
        home: const MainScreen(),
      ),
    );
  }
}

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  int _selectedIndex = 0;

  static const List<Widget> _screens = [
    DashboardScreen(),
    ConfigScreen(),
    TradeHistoryScreen(),
    ContractScreen(),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _screens[_selectedIndex],
      bottomNavigationBar: NavigationBar(
        selectedIndex: _selectedIndex,
        onDestinationSelected: (int index) {
          setState(() {
            _selectedIndex = index;
          });
        },
        destinations: const [
          NavigationDestination(
            icon: Icon(Icons.dashboard_outlined),
            selectedIcon: Icon(Icons.dashboard),
            label: 'Dashboard',
          ),
          NavigationDestination(
            icon: Icon(Icons.settings_outlined),
            selectedIcon: Icon(Icons.settings),
            label: 'Config',
          ),
          NavigationDestination(
            icon: Icon(Icons.history_outlined),
            selectedIcon: Icon(Icons.history),
            label: 'Trades',
          ),
          NavigationDestination(
            icon: Icon(Icons.account_balance_outlined),
            selectedIcon: Icon(Icons.account_balance),
            label: 'Contract',
          ),
        ],
      ),
    );
  }
}