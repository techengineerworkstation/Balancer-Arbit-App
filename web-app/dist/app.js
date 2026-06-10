const API_BASE = '';

// Network configurations
const NETWORKS = {
    arbitrum: {
        name: 'Arbitrum One',
        chainId: 42161,
        rpc: 'https://arb1.arbitrum.io/rpc',
        tokens: [
            { symbol: 'WETH', address: '0x82aF49447D8a07e3bd95BD0d56f35241523fBab1', color: 'token-eth', decimals: 18 },
            { symbol: 'USDC', address: '0xaf88d065e77c8cC2239327C5EDb3A432268e5831', color: 'token-usdc', decimals: 6 },
            { symbol: 'USDT', address: '0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9', color: 'token-usdt', decimals: 6 },
            { symbol: 'WBTC', address: '0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f', color: 'token-wbtc', decimals: 8 },
            { symbol: 'ARB', address: '0x912CE59144191C1204E64559FE8253a0e49E6548', color: 'token-arb', decimals: 18 },
            { symbol: 'GMX', address: '0xfc5A1A6EB076a2C7aD06eD22C90d7E710E35ad0a', color: 'token-gmx', decimals: 18 },
            { symbol: 'PENDLE', address: '0x0c880f6761F1af8d9Aa9C466984b80DAb9a8c9e8', color: 'token-pendle', decimals: 18 },
            { symbol: 'LINK', address: '0xf97f4df75117a78c1A5a0DBb814Af92458539FB4', color: 'token-link', decimals: 18 },
            { symbol: 'UNI', address: '0xFa7F8980b0f1E64A2062791cc3b0871572f1F7f0', color: 'token-uni', decimals: 18 },
            { symbol: 'AAVE', address: '0xba5ddd1f9d7F570dc94a51479a000e3bce967196', color: 'token-aave', decimals: 18 },
            { symbol: 'SUSHI', address: '0xd4d42f0b6def4ce0383636770ef773390d85c61a', color: 'token-sushi', decimals: 18 },
            { symbol: 'GNS', address: '0x18c11FD286C5EC11c3b683Caa813B77f5163A122', color: 'token-gns', decimals: 18 },
            { symbol: 'BAL', address: '0x040d1EdC9569d4Bab2D15287Dc5A4F10F56a56B8', color: 'token-bal', decimals: 18 },
            { symbol: 'MAGIC', address: '0x539bdE0d7Dbd336b79148AA742883198BBF60342', color: 'token-magic', decimals: 18 },
            { symbol: 'GRAIL', address: '0x3d9907F9a368ad0a51Be60f7Da3b97cf940982D8', color: 'token-grail', decimals: 18 },
            { symbol: 'DPEX', address: '0x6C2C06790b3E3E3c38e12Ee22F8183b37a13EE55', color: 'token-dpex', decimals: 18 },
            { symbol: 'RDNT', address: '0x0C4681e6C0235179ec3D4F4fc4DF3d14FDD96017', color: 'token-rdnt', decimals: 18 }
        ],
        dexes: [
            { name: 'SushiSwap V2', color: 'dex-sushi', key: 'sushiswap_v2' },
            { name: 'PancakeSwap V3', color: 'dex-pancake', key: 'pancakeswap_v3' },
            { name: 'Uniswap V3', color: 'dex-uniswap', key: 'uniswap_v3' },
            { name: 'Camelot V2', color: 'dex-camelot-v2', key: 'camelot_v2' },
            { name: 'Camelot V4', color: 'dex-camelot-v4', key: 'camelot_v4' },
            { name: 'Trader Joe V2.1', color: 'dex-trader-joe', key: 'trader_joe_v2' },
            { name: 'ZyberSwap V3', color: 'dex-zyber', key: 'zyber_v3' },
            { name: 'Ramses V3', color: 'dex-ramses', key: 'ramses_v3' },
            { name: 'SushiSwap V3', color: 'dex-sushiswap-v3', key: 'sushiswap_v3' }
        ]
    },
    base: {
        name: 'Base',
        chainId: 8453,
        rpc: 'https://mainnet.base.org',
        tokens: [
            { symbol: 'WETH', address: '0x4200000000000000000000000000000000000006', color: 'token-eth', decimals: 18 },
            { symbol: 'USDC', address: '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913', color: 'token-usdc', decimals: 6 },
            { symbol: 'DAI', address: '0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb', color: 'token-dai', decimals: 18 },
            { symbol: 'cbETH', address: '0x2Ae3F1Ec7F1F5012CFEab0185bfc7aa3cf0DEc22', color: 'token-eth', decimals: 18 },
            { symbol: 'wstETH', address: '0xc1CBa3fCea344f92D9239c08C0568f6F2F0ee452', color: 'token-eth', decimals: 18 },
            { symbol: 'AERO', address: '0x940181a94A35A4569E4529A3CDfB74e38FD98631', color: 'token-aero', decimals: 18 },
            { symbol: 'VIRTUAL', address: '0x0b3e3c8e9f63630d9576150d059736f6c93592e6', color: 'token-magic', decimals: 18 },
            { symbol: 'DEGEN', address: '0x4ed4E862860beD51a9570b96d89aF5E1B0Efefed', color: 'token-gmx', decimals: 18 },
            { symbol: 'BRETT', address: '0x532f27101965dd16442E59d40670FaF5eBB142E4', color: 'token-sushi', decimals: 18 },
            { symbol: 'ANDY', address: '0xf5Cb8DBA2a7BE282b268520F3B0c41C4d5A1d3c3', color: 'token-pendle', decimals: 18 },
            { symbol: 'MORPHO', address: '0xbaa0FDFc42031DAcE800384dD3C9C4A0C10c6D57', color: 'token-bal', decimals: 18 }
        ],
        dexes: [
            { name: 'Uniswap V3', color: 'dex-uniswap', key: 'uniswap_v3' },
            { name: 'Aerodrome V2', color: 'dex-aerodrome', key: 'aerodrome_v2' },
            { name: 'BaseSwap V2', color: 'dex-baseswap', key: 'baseswap_v2' }
        ]
    },
    optimism: {
        name: 'Optimism',
        chainId: 10,
        rpc: 'https://mainnet.optimism.io',
        tokens: [
            { symbol: 'WETH', address: '0x4200000000000000000000000000000000000006', color: 'token-eth', decimals: 18 },
            { symbol: 'USDC', address: '0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85', color: 'token-usdc', decimals: 6 },
            { symbol: 'USDT', address: '0x94b008aA00579c1307B0EF2c499aD98a8ce58e58', color: 'token-usdt', decimals: 6 },
            { symbol: 'DAI', address: '0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1', color: 'token-dai', decimals: 18 },
            { symbol: 'OP', address: '0x4200000000000000000000000000000000000042', color: 'token-op', decimals: 18 },
            { symbol: 'VELO', address: '0x9560e827aF36c94D2Ac33a39bCE1Fe78631088Db', color: 'token-velo', decimals: 18 },
            { symbol: 'wstETH', address: '0x1F32b1c2345538c0c6f582fCB022739c4A194Ebb', color: 'token-eth', decimals: 18 },
            { symbol: 'SNX', address: '0x8700dAec35aF8Ff88c16BdF0418774CB3D7599B4', color: 'token-bal', decimals: 18 },
            { symbol: 'AAVE', address: '0x76FB31fb4af56892A25e32cFC43De717950c9278', color: 'token-aave', decimals: 18 },
            { symbol: 'LINK', address: '0x350a791Bfc6C61f2c36F2E10bc31c720766892cE', color: 'token-link', decimals: 18 },
            { symbol: 'UNI', address: '0x6fd9d7AD17242c41f7131d257212c54A0e816691', color: 'token-uni', decimals: 18 },
            { symbol: 'PERP', address: '0x9e1028F5F1D5eDE59748FFceE5532509976840E0', color: 'token-gmx', decimals: 18 }
        ],
        dexes: [
            { name: 'Uniswap V3', color: 'dex-uniswap', key: 'uniswap_v3' },
            { name: 'Velodrome V2', color: 'dex-velodrome', key: 'velodrome_v2' },
            { name: 'SushiSwap V2', color: 'dex-sushi', key: 'sushiswap_v2' }
        ]
    }
};

let currentNetwork = 'arbitrum';
let TOKENS = NETWORKS.arbitrum.tokens;
let DEXES = NETWORKS.arbitrum.dexes;
const QUICK_AMOUNTS = [100, 500, 1000, 5000, 10000, 25000, 50000, 100000];

let authToken = localStorage.getItem('arb_token') || null;
let currentUser = localStorage.getItem('arb_user') || null;
let sseSource = null;
let botRunning = false;
let botStatus = {};
let feeData = {};
let configData = {};
let tradesData = [];
let opportunitiesData = [];
let scanProgress = null;
let sseConnected = false;
let activeTokens = TOKENS.map(t => t.symbol);
let activeDexes = DEXES.map(d => d.name);

function formatUSD(val) {
    if (val === null || val === undefined) return '$0';
    const num = Number(val);
    if (num >= 1e9) return '$' + (num / 1e9).toFixed(2) + 'B';
    if (num >= 1e6) return '$' + (num / 1e6).toFixed(2) + 'M';
    if (num >= 1e3) return '$' + (num / 1e3).toFixed(1) + 'K';
    return '$' + num.toFixed(2);
}

function switchNetwork(network) {
    if (!NETWORKS[network]) return;
    currentNetwork = network;
    TOKENS = NETWORKS[network].tokens;
    DEXES = NETWORKS[network].dexes;
    activeTokens = TOKENS.map(t => t.symbol);
    activeDexes = DEXES.map(d => d.name);
    
    // Update UI
    document.querySelectorAll('.network-btn').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.network === network);
    });
    document.getElementById('network-badge').textContent = NETWORKS[network].name;
    
    // Re-render badges
    renderTokenBadges();
    renderDEXBadges();
    
    // Clear old data and reconnect SSE
    opportunitiesData = [];
    disconnectSSE();
    connectSSE();
}

function formatNumber(val) {
    if (val === null || val === undefined) return '0';
    const num = Number(val);
    if (num >= 1e9) return (num / 1e9).toFixed(2) + 'B';
    if (num >= 1e6) return (num / 1e6).toFixed(2) + 'M';
    if (num >= 1e3) return num.toLocaleString('en-US', { maximumFractionDigits: 0 });
    return num.toFixed(2);
}

function timeAgo(ts) {
    if (!ts) return '';
    const diff = Date.now() - new Date(ts).getTime();
    if (diff < 60000) return Math.floor(diff / 1000) + 's ago';
    if (diff < 3600000) return Math.floor(diff / 60000) + 'm ago';
    if (diff < 86400000) return Math.floor(diff / 3600000) + 'h ago';
    return new Date(ts).toLocaleDateString();
}

function showError(msg) {
    const banner = document.getElementById('error-banner');
    if (!banner) return;
    banner.textContent = msg;
    banner.style.display = 'flex';
    setTimeout(() => { banner.textContent = ''; banner.style.display = 'none'; }, 8000);
}

async function apiFetch(path, opts = {}) {
    const headers = { 'Content-Type': 'application/json' };
    if (authToken) headers['Authorization'] = 'Bearer ' + authToken;
    const resp = await fetch(API_BASE + path, { ...opts, headers: { ...headers, ...opts.headers } });
    if (resp.status === 401) { logout(); return null; }
    if (!resp.ok) {
        const err = await resp.json().catch(() => ({ error: resp.statusText }));
        throw new Error(err.error || err.message || 'Request failed');
    }
    return await resp.json();
}

function renderTokenBadge(symbol) {
    const t = TOKENS.find(x => x.symbol === symbol);
    if (!t) return `<span class="badge badge-sm">${symbol}</span>`;
    return `<span class="badge badge-sm ${t.color}">${t.symbol}</span>`;
}

function renderDexBadge(name) {
    const d = DEXES.find(x => x.name === name);
    if (!d) return `<span class="badge badge-sm">${name}</span>`;
    return `<span class="badge badge-sm ${d.color}">${d.name}</span>`;
}

function getTokenSymbol(addr) {
    const t = TOKENS.find(x => x.address && x.address.toLowerCase() === (addr || '').toLowerCase());
    return t ? t.symbol : (addr ? addr.slice(0, 6) + '...' : '-');
}

function parsePair(trade) {
    if (trade.pair) return trade.pair;
    if (trade.token_pair) return trade.token_pair;
    if (trade.tokenA && trade.tokenB) return getTokenSymbol(trade.tokenA) + '/' + getTokenSymbol(trade.tokenB);
    return '-';
}

function parseDexIn(trade) {
    if (trade.dex_in) return trade.dex_in;
    if (trade.dexIn) return trade.dexIn;
    if (trade.buy_dex) return trade.buy_dex;
    return trade.route_in || '-';
}

function parseDexOut(trade) {
    if (trade.dex_out) return trade.dex_out;
    if (trade.dexOut) return trade.dexOut;
    if (trade.sell_dex) return trade.sell_dex;
    return trade.route_out || '-';
}

function checkAuth() {
    if (authToken && currentUser) {
        document.getElementById('auth-page').style.display = 'none';
        document.getElementById('app-page').style.display = 'block';
        document.getElementById('user-display').textContent = currentUser;
        initApp();
    } else {
        document.getElementById('auth-page').style.display = 'flex';
        document.getElementById('app-page').style.display = 'none';
    }
}

function showAuthTab(tab) {
    document.querySelectorAll('.auth-tab').forEach(t => t.classList.remove('active'));
    event.target.classList.add('active');
    document.getElementById('login-form').style.display = tab === 'login' ? 'flex' : 'none';
    document.getElementById('register-form').style.display = tab === 'register' ? 'flex' : 'none';
    document.getElementById('auth-error').classList.remove('show');
}

async function handleLogin(e) {
    e.preventDefault();
    const username = document.getElementById('login-username').value.trim();
    const password = document.getElementById('login-password').value;
    const errEl = document.getElementById('auth-error');
    errEl.classList.remove('show');
    try {
        const data = await fetch(API_BASE + '/api/auth/login', {
            method: 'POST', headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password })
        }).then(r => r.json());
        if (data.token) {
            authToken = data.token;
            currentUser = data.username || username;
            localStorage.setItem('arb_token', authToken);
            localStorage.setItem('arb_user', currentUser);
            checkAuth();
        } else {
            errEl.textContent = data.error || 'Login failed';
            errEl.classList.add('show');
        }
    } catch (err) {
        errEl.textContent = 'Connection error: ' + err.message;
        errEl.classList.add('show');
    }
}

async function handleRegister(e) {
    e.preventDefault();
    const username = document.getElementById('reg-username').value.trim();
    const password = document.getElementById('reg-password').value;
    const password2 = document.getElementById('reg-password2').value;
    const errEl = document.getElementById('auth-error');
    errEl.classList.remove('show');
    if (password !== password2) {
        errEl.textContent = 'Passwords do not match';
        errEl.classList.add('show');
        return;
    }
    try {
        const data = await fetch(API_BASE + '/api/auth/register', {
            method: 'POST', headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password })
        }).then(r => r.json());
        if (data.token) {
            authToken = data.token;
            currentUser = data.username || username;
            localStorage.setItem('arb_token', authToken);
            localStorage.setItem('arb_user', currentUser);
            checkAuth();
        } else {
            errEl.textContent = data.error || 'Registration failed';
            errEl.classList.add('show');
        }
    } catch (err) {
        errEl.textContent = 'Connection error: ' + err.message;
        errEl.classList.add('show');
    }
}

function handleLogout() {
    localStorage.removeItem('arb_token');
    localStorage.removeItem('arb_user');
    authToken = null;
    currentUser = null;
    disconnectSSE();
    checkAuth();
}

function logout() { handleLogout(); }

function connectSSE() {
    if (sseSource) sseSource.close();
    sseSource = new EventSource(API_BASE + '/api/events');

    sseSource.onopen = () => {
        sseConnected = true;
        updateSSEStatus(true);
    };

    sseSource.onerror = () => {
        const state = sseSource ? sseSource.readyState : -1;
        if (state === EventSource.CLOSED) {
            sseConnected = false;
            updateSSEStatus(false);
            setTimeout(connectSSE, 5000);
        } else if (state === EventSource.CONNECTING) {
            updateSSEStatus(false);
        }
    };

    sseSource.onmessage = (event) => {
        try {
            const msg = JSON.parse(event.data);
            sseConnected = true;
            updateSSEStatus(true);

            switch (msg.type) {
                case 'status':
                    botStatus = msg.data || {};
                    updateStatusGrid();
                    updateControls();
                    updateScanStatus();
                    break;
                case 'trade':
                    tradesData.unshift(msg.data || msg);
                    if (tradesData.length > 100) tradesData.pop();
                    renderTradesTable();
                    break;
                case 'fees':
                    feeData = msg.data || msg;
                    renderFeeGrid();
                    break;
                case 'config':
                    configData = msg.data || msg;
                    if (configData.active_tokens) activeTokens = configData.active_tokens;
                    if (configData.active_dexes) activeDexes = configData.active_dexes;
                    renderTokenBadges();
                    renderDEXBadges();
                    renderConfigForm();
                    break;
                case 'opportunity':
                    handleOpportunity(msg.data || msg);
                    break;
                case 'scan':
                    handleScanUpdate(msg.data || msg);
                    break;
                case 'pair_scan':
                    updateScanStatus(msg.data || msg);
                    break;
                case 'heartbeat':
                    sseConnected = true;
                    updateSSEStatus(true);
                    break;
            }
        } catch {}
    };

    sseSource.addEventListener('status', (e) => {
        try {
            sseConnected = true;
            updateSSEStatus(true);
            botStatus = JSON.parse(e.data);
            updateStatusGrid();
            updateControls();
            updateScanStatus();
        } catch {}
    });

    sseSource.addEventListener('trade', (e) => {
        try {
            sseConnected = true;
            updateSSEStatus(true);
            const trade = JSON.parse(e.data);
            tradesData.unshift(trade);
            if (tradesData.length > 100) tradesData.pop();
            renderTradesTable();
        } catch {}
    });

    sseSource.addEventListener('fees', (e) => {
        try {
            sseConnected = true;
            updateSSEStatus(true);
            feeData = JSON.parse(e.data);
            renderFeeGrid();
        } catch {}
    });

    sseSource.addEventListener('config', (e) => {
        try {
            sseConnected = true;
            updateSSEStatus(true);
            configData = JSON.parse(e.data);
            if (configData.active_tokens) activeTokens = configData.active_tokens;
            if (configData.active_dexes) activeDexes = configData.active_dexes;
            renderTokenBadges();
            renderDEXBadges();
            renderConfigForm();
        } catch {}
    });

    sseSource.addEventListener('heartbeat', () => {
        sseConnected = true;
        updateSSEStatus(true);
    });

    sseSource.addEventListener('pair_scan', (e) => {
        try {
            const data = JSON.parse(e.data);
            updateScanStatus(data);
        } catch {}
    });

    sseSource.addEventListener('opportunity', (e) => {
        try {
            sseConnected = true;
            updateSSEStatus(true);
            handleOpportunity(JSON.parse(e.data));
        } catch {}
    });

    sseSource.addEventListener('scan', (e) => {
        try {
            sseConnected = true;
            updateSSEStatus(true);
            handleScanUpdate(JSON.parse(e.data));
        } catch {}
    });
}

function disconnectSSE() {
    if (sseSource) { sseSource.close(); sseSource = null; }
    sseConnected = false;
    updateSSEStatus(false);
}

function updateSSEStatus(connected) {
    const el = document.getElementById('sse-status');
    if (!el) return;
    if (connected) {
        el.className = 'sse-status connected';
        el.textContent = 'Live';
    } else {
        el.className = 'sse-status disconnected';
        el.textContent = 'Offline';
    }
}

function handleOpportunity(data) {
    if (!data) return;
    const profit = Number(data.net_profit_after_costs || data.expected_profit || data.profit || 0);
    if (profit <= 0) return;
    const opp = {
        pair: data.token_pair || data.pair || '-',
        dex_in: data.dex_from || data.dex_in || '-',
        dex_out: data.dex_to || data.dex_out || '-',
        profit: profit,
        amount: Number(data.amount || data.loan_amount || 0),
        timestamp: data.timestamp || new Date().toISOString(),
        route: data.route_description || data.route || '',
        slippage_bps: data.slippage_bps || 0,
        price_impact_bps: data.price_impact_bps || 0
    };

    opportunitiesData.unshift(opp);
    if (opportunitiesData.length > 50) opportunitiesData.pop();
    renderOpportunities();
}

function handleScanUpdate(data) {
    scanProgress = data;
    renderScanProgress();
    if (data.pairs || data.pair) {
        updateScanStatus(data);
    }
}

function renderOpportunities() {
    const list = document.getElementById('opportunities-list');
    const counter = document.getElementById('opp-count');
    if (!list) return;

    if (counter) counter.textContent = opportunitiesData.length;

    if (opportunitiesData.length === 0) {
        list.innerHTML = '<div class="empty-state-small">Waiting for opportunities...</div>';
        return;
    }

    list.innerHTML = opportunitiesData.slice(0, 15).map(opp => {
        const profit = opp.profit;
        const profitClass = profit >= 0 ? 'opp-profit-positive' : 'opp-profit-negative';
        const profitPct = opp.profit_pct ? `<span class="opp-pct">${opp.profit_pct.toFixed(2)}%</span>` : '';
        const pairParts = opp.pair.split('/').map(s => s.trim());
        const pairHtml = pairParts.length > 1
            ? `${renderTokenBadge(pairParts[0])}<span class="opp-pair-sep">/</span>${renderTokenBadge(pairParts[1])}`
            : `<span class="badge badge-sm">${opp.pair}</span>`;

        return `<div class="opp-card">
            <div class="opp-header">
                <div class="opp-pair">${pairHtml}</div>
                <div class="opp-time">${timeAgo(opp.timestamp)}</div>
            </div>
            <div class="opp-body">
                <div class="opp-route">
                    <span class="opp-dex">${renderDexBadge(opp.dex_in)}</span>
                    <span class="opp-arrow">&rarr;</span>
                    <span class="opp-dex">${renderDexBadge(opp.dex_out)}</span>
                </div>
                <div class="opp-profit ${profitClass}">
                    <span class="opp-profit-val">${formatUSD(profit)}</span>
                    ${profitPct}
                </div>
            </div>
        </div>`;
    }).join('');
}

function renderScanProgress() {
    const bar = document.getElementById('scan-progress-bar');
    if (!bar || !scanProgress) return;

    if (scanProgress.progress !== undefined) {
        bar.style.width = Math.min(100, scanProgress.progress) + '%';
        bar.style.display = 'block';
    } else {
        bar.style.width = '0%';
        bar.style.display = 'none';
    }
}

function initApp() {
    renderTokenBadges();
    renderDEXBadges();
    renderLoanAmountBar();
    renderFeeGrid();
    renderTradesTable();
    renderOpportunities();
    connectSSE();
    fetchInitialData();
    setupPageNav();
}

async function fetchInitialData() {
    try {
        const fees = await apiFetch('/api/fees');
        if (fees) { feeData = fees; renderFeeGrid(); renderContractFeeGrid(); }
    } catch {}
    try {
        const cfg = await apiFetch('/api/config');
        if (cfg) {
            configData = cfg;
            if (configData.active_tokens) activeTokens = configData.active_tokens;
            if (configData.active_dexes) activeDexes = configData.active_dexes;
            renderConfigForm();
        }
    } catch {}
    try {
        const status = await apiFetch('/api/status');
        if (status) { botStatus = status; updateStatusGrid(); updateControls(); updateScanStatus(); }
    } catch {}
    try {
        const trades = await apiFetch('/api/trades');
        if (Array.isArray(trades)) { tradesData = trades; renderTradesTable(); }
    } catch {}
}

function renderFeeGrid() {
    const grid = document.getElementById('fee-grid');
    if (!grid) return;
    const fees = [
        { label: 'Gas Price', value: (feeData.gas_price_gwei || feeData.gas_price || 0).toFixed(2) + ' Gwei' },
        { label: 'Swap Fee', value: formatUSD(feeData.swap_fee_usd || feeData.swap_fee || 0), highlight: true },
        { label: 'L1 Data Fee', value: formatUSD(feeData.l1_data_fee_usd || feeData.l1_data_fee || 0) },
        { label: 'Flash Loan Fee', value: feeData.flash_loan_fee_usd || feeData.flash_loan_fee ? formatUSD(feeData.flash_loan_fee_usd || feeData.flash_loan_fee) : 'Free (0%)' },
        { label: 'Deployment Cost', value: formatUSD(feeData.deployment_cost_usd || feeData.deployment_cost || 0) },
        { label: 'Total Per-Trade', value: formatUSD(feeData.total_fee_per_trade_usd || feeData.total_per_trade || 0), highlight: true }
    ];
    grid.innerHTML = fees.map(f =>
        `<div class="fee-card ${f.highlight ? 'highlight' : ''}">
            <div class="fee-label">${f.label}</div>
            <div class="fee-value">${f.value}</div>
        </div>`
    ).join('');
}

function renderContractFeeGrid() {
    const grid = document.getElementById('contract-fee-grid');
    if (!grid) return;
    const gasPrice = feeData.gas_price_gwei || 0.02;
    const deployGas = 2500000;
    const ethPrice = 2500;
    const deployCostEth = (gasPrice * deployGas) / 1e9;
    const deployCostUsd = deployCostEth * ethPrice;
    
    const items = [
        { label: 'Estimated Deploy Cost', value: formatUSD(deployCostUsd) },
        { label: 'Gas Price', value: gasPrice.toFixed(2) + ' Gwei' },
        { label: 'Est. Deploy Gas', value: '~2,500,000 units' },
        { label: 'L2 Fee', value: formatUSD(deployCostUsd * 0.01) },
        { label: 'Network', value: 'Arbitrum One (42161)' },
        { label: 'Flash Loan Pool', value: 'Balancer V3 Vault' }
    ];
    grid.innerHTML = items.map(f => `<div class="fee-card">
        <div class="fee-label">${f.label}</div>
        <div class="fee-value">${f.value}</div>
    </div>`).join('');
}

function renderTokenBadges() {
    const container = document.getElementById('token-badges');
    const counter = document.getElementById('token-count');
    if (!container) return;
    container.innerHTML = TOKENS.map(t => {
        const active = activeTokens.includes(t.symbol);
        return `<span class="badge ${t.color} ${active ? '' : 'badge-inactive'}">${t.symbol}</span>`;
    }).join('');
    if (counter) counter.textContent = `(${activeTokens.length}/${TOKENS.length})`;
}

function renderDEXBadges() {
    const container = document.getElementById('dex-badges');
    const counter = document.getElementById('dex-count');
    if (!container) return;
    container.innerHTML = DEXES.map(d => {
        const active = activeDexes.includes(d.name);
        return `<span class="badge ${d.color} ${active ? '' : 'badge-inactive'}">${d.name}</span>`;
    }).join('');
    if (counter) counter.textContent = `(${activeDexes.length}/${DEXES.length})`;
}

function renderLoanAmountBar() {
    const slider = document.getElementById('loan-slider');
    const input = document.getElementById('loan-input');
    const display = document.getElementById('loan-display');
    const quickDiv = document.getElementById('quick-amounts');
    if (!slider || !input || !display || !quickDiv) return;

    function update(val) {
        const v = Math.min(500000, Math.max(100, Number(val)));
        slider.value = v;
        input.value = v;
        display.textContent = formatUSD(v);
        document.querySelectorAll('.quick-amount-btn').forEach(btn => {
            btn.classList.toggle('active', Number(btn.dataset.amount) === v);
        });
    }

    slider.addEventListener('input', () => update(slider.value));
    input.addEventListener('input', () => update(input.value));
    quickDiv.innerHTML = QUICK_AMOUNTS.map(a =>
        `<button class="quick-amount-btn" data-amount="${a}" onclick="setLoanAmount(${a})">${formatUSD(a)}</button>`
    ).join('');
}

function setLoanAmount(val) {
    const slider = document.getElementById('loan-slider');
    const input = document.getElementById('loan-input');
    if (slider) slider.value = val;
    if (input) input.value = val;
    document.getElementById('loan-display').textContent = formatUSD(val);
    document.querySelectorAll('.quick-amount-btn').forEach(btn => {
        btn.classList.toggle('active', Number(btn.dataset.amount) === val);
    });
}

function updateScanStatus(data) {
    const el = document.getElementById('scan-pairs');
    const indicator = document.getElementById('scan-indicator');
    if (!el) return;

    if (indicator) {
        indicator.className = botStatus.running ? 'scan-indicator scanning' : 'scan-indicator';
    }

    if (data && data.pairs) {
        el.innerHTML = data.pairs.slice(0, 8).map(p => {
            const symbols = p.split('/');
            return `<span class="scan-pair">${symbols.map(s => renderTokenBadge(s)).join('<span class="scan-arrow">/</span>')}</span>`;
        }).join('');
    } else if (data && data.pair) {
        el.innerHTML = `<span class="scan-pair">${data.pair}</span>`;
    } else {
        const pairs = botStatus.scanning_pairs || botStatus.active_pairs_list || [];
        if (pairs.length > 0) {
            el.innerHTML = pairs.slice(0, 6).map(p => `<span class="scan-pair">${p}</span>`).join('');
        } else {
            el.textContent = botStatus.running ? 'Scanning all token pairs across ' + activeDexes.length + ' DEXes...' : 'Bot stopped';
        }
    }
}

function updateStatusGrid() {
    const grid = document.getElementById('status-grid');
    if (!grid) return;
    const running = botStatus.is_running || botStatus.running;
    const cards = [
        { label: 'Bot Status', value: running ? 'Running' : 'Stopped', state: running ? 'live' : 'idle' },
        { label: 'Network', value: botStatus.network || 'Arbitrum One', state: 'live' },
        { label: 'Gas Price', value: (botStatus.current_gas_price_gwei || feeData.gas_price_gwei || 0) + ' Gwei', state: 'idle' },
        { label: 'ETH Price', value: formatUSD(botStatus.eth_price || 2500), state: 'idle' },
        { label: 'Total Scans', value: formatNumber(botStatus.total_scans || 0), state: 'idle' },
        { label: 'Opportunities Found', value: botStatus.opportunities_found || 0, state: (botStatus.opportunities_found || 0) > 0 ? 'live' : 'idle' },
        { label: 'Scanning', value: botStatus.scanning ? 'Active' : 'Idle', state: botStatus.scanning ? 'live' : 'idle' },
        { label: 'Pairs Scanned', value: botStatus.pairs_scanned || 0, state: 'idle' },
        { label: 'Total Profit', value: formatUSD(botStatus.total_profit || 0), state: (botStatus.total_profit || 0) > 0 ? 'live' : 'idle' },
        { label: 'Trades Executed', value: botStatus.total_trades || 0, state: 'idle' },
        { label: 'Avg Slippage', value: (botStatus.avg_slippage_bps || 0) + ' bps', state: 'idle' },
        { label: 'Avg Price Impact', value: (botStatus.avg_price_impact_bps || 0) + ' bps', state: 'idle' },
        { label: 'Balance', value: formatUSD(botStatus.balance || 0), state: 'idle' },
        { label: 'Last Scan', value: botStatus.last_scan ? timeAgo(botStatus.last_scan) : 'N/A', state: 'idle' },
        { label: 'Flash Loan Pool', value: 'Balancer V3', state: 'idle' },
        { label: 'Scan Interval', value: (botStatus.scan_interval_ms || 100) + 'ms', state: 'idle' }
    ];
    grid.innerHTML = cards.map(c =>
        `<div class="status-card ${c.state}">
            <div class="status-label">${c.label}</div>
            <div class="status-value">${c.value}</div>
        </div>`
    ).join('');
}

function updateControls() {
    const startBtn = document.getElementById('btn-start');
    const stopBtn = document.getElementById('btn-stop');
    const running = botStatus.is_running || botStatus.running;
    if (startBtn) startBtn.disabled = running;
    if (stopBtn) stopBtn.disabled = !running;
    botRunning = !!running;
}

async function startBot() {
    const startBtn = document.getElementById('btn-start');
    if (startBtn) startBtn.disabled = true;
    try { await apiFetch('/api/bot/start', { method: 'POST' }); }
    catch (e) { showError('Failed to start bot: ' + e.message); if (startBtn) startBtn.disabled = false; }
}

async function stopBot() {
    const stopBtn = document.getElementById('btn-stop');
    if (stopBtn) stopBtn.disabled = true;
    try { await apiFetch('/api/bot/stop', { method: 'POST' }); }
    catch (e) { showError('Failed to stop bot: ' + e.message); if (stopBtn) stopBtn.disabled = false; }
}

async function executeTrade() {
    const btn = document.getElementById('btn-execute');
    if (btn) btn.disabled = true;
    const loanAmount = document.getElementById('loan-input')?.value || 10000;
    try {
        const result = await apiFetch('/api/trade/execute', {
            method: 'POST', body: JSON.stringify({ loan_amount: Number(loanAmount) })
        });
        if (result && result.error) showError(result.error);
    } catch (e) { showError('Trade execution failed: ' + e.message); }
    finally { if (btn) btn.disabled = false; }
}

async function simulateTrade() {
    const btn = document.getElementById('btn-simulate');
    if (btn) btn.disabled = true;
    const loanAmount = document.getElementById('loan-input')?.value || 10000;
    try {
        const result = await apiFetch('/api/trade/simulate', {
            method: 'POST', body: JSON.stringify({ loan_amount: Number(loanAmount) })
        });
        const simResult = document.getElementById('sim-result');
        const simText = document.getElementById('sim-result-text');
        if (simResult && simText) {
            simResult.style.display = 'block';
            simText.textContent = JSON.stringify(result || { status: 'No result' }, null, 2);
        }
    } catch (e) { showError('Simulation failed: ' + e.message); }
    finally { if (btn) btn.disabled = false; }
}

function renderTradesTable() {
    const tbody = document.getElementById('trades-tbody');
    if (!tbody) return;
    const trades = tradesData.slice(0, 20);
    if (trades.length === 0) {
        tbody.innerHTML = `<tr><td colspan="8" class="empty-state">No trades yet. Start the bot or execute a manual trade.</td></tr>`;
        return;
    }
    tbody.innerHTML = trades.map(t => {
        const profit = Number(t.profit || 0);
        const profitClass = profit >= 0 ? 'profit-positive' : 'profit-negative';
        let statusClass = 'status-pending';
        if (t.status === 'success' || t.status === 'completed') statusClass = 'status-success';
        else if (t.status === 'failed' || t.status === 'error') statusClass = 'status-fail';
        const pair = parsePair(t);
        const dexIn = parseDexIn(t);
        const dexOut = parseDexOut(t);
        return `<tr>
            <td>${timeAgo(t.timestamp || t.time)}</td>
            <td class="td-pair">${pair.split('/').map(s => renderTokenBadge(s.trim())).join('')}</td>
            <td class="td-mono">${formatUSD(t.amount || t.loan_amount)}</td>
            <td>${renderDexBadge(dexIn)}</td>
            <td>${renderDexBadge(dexOut)}</td>
            <td class="${profitClass} td-mono">${formatUSD(profit)}</td>
            <td class="td-mono">${formatUSD(t.gas_cost || t.gas)}</td>
            <td><span class="${statusClass}">${t.status || 'pending'}</span></td>
        </tr>`;
    }).join('');

    const fullList = document.getElementById('full-trades-list');
    if (fullList) {
        if (tradesData.length === 0) {
            fullList.innerHTML = `<div class="empty-state"><p>No trade history yet.</p></div>`;
        } else {
            fullList.innerHTML = `<div class="table-scroll"><table class="trades-table"><thead><tr>
                <th>Time</th><th>Pair</th><th>Amount</th>
                <th>DEX In</th><th>DEX Out</th><th>Profit</th>
                <th>Gas</th><th>Status</th>
            </tr></thead><tbody>${tradesData.map(t => {
                const profit = Number(t.profit || 0);
                const profitClass = profit >= 0 ? 'profit-positive' : 'profit-negative';
                let statusClass = 'status-pending';
                if (t.status === 'success' || t.status === 'completed') statusClass = 'status-success';
                else if (t.status === 'failed' || t.status === 'error') statusClass = 'status-fail';
                const pair = parsePair(t);
                const dexIn = parseDexIn(t);
                const dexOut = parseDexOut(t);
                return `<tr>
                    <td>${timeAgo(t.timestamp || t.time)}</td>
                    <td class="td-pair">${pair.split('/').map(s => renderTokenBadge(s.trim())).join('')}</td>
                    <td class="td-mono">${formatUSD(t.amount || t.loan_amount)}</td>
                    <td>${renderDexBadge(dexIn)}</td>
                    <td>${renderDexBadge(dexOut)}</td>
                    <td class="${profitClass} td-mono">${formatUSD(profit)}</td>
                    <td class="td-mono">${formatUSD(t.gas_cost || t.gas)}</td>
                    <td><span class="${statusClass}">${t.status || 'pending'}</span></td>
                </tr>`;
            }).join('')}</tbody></table></div>`;
        }
    }
}

function renderConfigForm() {
    const container = document.getElementById('config-form');
    const tokenGrid = document.getElementById('config-token-grid');
    const dexGrid = document.getElementById('config-dex-grid');
    if (tokenGrid) {
        tokenGrid.innerHTML = `<div class="config-section">
            <h3>Active Tokens</h3>
            <div class="checkbox-grid">${TOKENS.map(t => {
                const checked = activeTokens.includes(t.symbol) ? 'checked' : '';
                return `<div class="config-checkbox">
                    <input type="checkbox" id="tok-${t.symbol}" ${checked} data-token="${t.symbol}" onchange="toggleToken('${t.symbol}', this.checked)">
                    <label for="tok-${t.symbol}" class="${t.color}">${t.symbol}</label>
                </div>`;
            }).join('')}</div>
        </div>`;
    }
    if (dexGrid) {
        dexGrid.innerHTML = `<div class="config-section">
            <h3>Active DEXes</h3>
            <div class="checkbox-grid">${DEXES.map(d => {
                const checked = activeDexes.includes(d.name) ? 'checked' : '';
                return `<div class="config-checkbox">
                    <input type="checkbox" id="dex-${d.key}" ${checked} data-dex="${d.name}" onchange="toggleDex('${d.name}', this.checked)">
                    <label for="dex-${d.key}" class="${d.color}">${d.name}</label>
                </div>`;
            }).join('')}</div>
        </div>`;
    }
    if (!container) return;
    const sections = [
        {
            title: 'General',
            fields: [
                { key: 'rpc_url', label: 'RPC URL', type: 'text', value: configData.rpc_url || 'https://arb1.arbitrum.io/rpc' },
                { key: 'private_key', label: 'Private Key', type: 'password', value: configData.private_key || '' },
                { key: 'contract_address', label: 'Contract Address', type: 'text', value: configData.contract_address || '' },
                { key: 'chain_id', label: 'Chain ID', type: 'number', value: configData.chain_id || 42161 }
            ]
        },
        {
            title: 'Trading Parameters',
            fields: [
                { key: 'min_profit_threshold', label: 'Min Profit Threshold ($)', type: 'number', value: configData.min_profit_threshold || 5 },
                { key: 'max_loan_amount', label: 'Max Loan Amount ($)', type: 'number', value: configData.max_loan_amount || 100000 },
                { key: 'max_slippage', label: 'Max Slippage (%)', type: 'number', value: configData.max_slippage || 0.5 },
                { key: 'scan_interval', label: 'Scan Interval (ms)', type: 'number', value: configData.scan_interval || 500 },
                { key: 'max_gas_price', label: 'Max Gas Price (Gwei)', type: 'number', value: configData.max_gas_price || 50 }
            ]
        },
        {
            title: 'DEX Router Addresses',
            fields: DEXES.map(d => ({
                key: d.key + '_router', label: d.name + ' Router', type: 'text', value: configData[d.key + '_router'] || ''
            })).concat([
                { key: 'balancer_vault', label: 'Balancer V3 Vault', type: 'text', value: configData.balancer_vault || '' }
            ])
        },
        {
            title: 'Auto-Execute',
            fields: [
                { key: 'auto_execute', label: 'Auto-Execute Trades', type: 'checkbox', value: configData.auto_execute || false },
                { key: 'simulate_before_exec', label: 'Simulate Before Execute', type: 'checkbox', value: configData.simulate_before_exec !== false },
                { key: 'multi_hop_enabled', label: 'Multi-Hop Routing', type: 'checkbox', value: configData.multi_hop_enabled !== false }
            ]
        }
    ];

    container.innerHTML = sections.map(s => `
        <div class="config-section">
            <h3>${s.title}</h3>
            ${s.fields.map(f => {
                if (f.type === 'checkbox') {
                    return `<div class="config-checkbox">
                        <input type="checkbox" id="cfg-${f.key}" ${f.value ? 'checked' : ''} data-key="${f.key}">
                        <label for="cfg-${f.key}">${f.label}</label>
                    </div>`;
                }
                return `<div class="config-field">
                    <label for="cfg-${f.key}">${f.label}</label>
                    <input type="${f.type}" id="cfg-${f.key}" value="${f.value}" data-key="${f.key}">
                </div>`;
            }).join('')}
        </div>
    `).join('') + `
        <div class="config-save-bar">
            <button class="btn btn-primary" onclick="saveConfig()">Save Configuration</button>
            <button class="btn btn-secondary" onclick="resetConfig()">Reset to Defaults</button>
        </div>
    `;
}

function toggleToken(symbol, checked) {
    if (checked) {
        if (!activeTokens.includes(symbol)) activeTokens.push(symbol);
    } else {
        activeTokens = activeTokens.filter(s => s !== symbol);
    }
    renderTokenBadges();
    saveActiveConfig();
}

function toggleDex(name, checked) {
    if (checked) {
        if (!activeDexes.includes(name)) activeDexes.push(name);
    } else {
        activeDexes = activeDexes.filter(n => n !== name);
    }
    renderDEXBadges();
    saveActiveConfig();
}

async function saveActiveConfig() {
    try {
        await apiFetch('/api/config', {
            method: 'POST',
            body: JSON.stringify({ active_tokens: activeTokens, active_dexes: activeDexes })
        });
    } catch {}
}

async function saveConfig() {
    const config = { active_tokens: activeTokens, active_dexes: activeDexes };
    document.querySelectorAll('#config-form [data-key]').forEach(el => {
        const key = el.dataset.key;
        if (el.type === 'checkbox') config[key] = el.checked;
        else if (el.type === 'number') config[key] = Number(el.value);
        else config[key] = el.value;
    });
    try {
        await apiFetch('/api/config', { method: 'POST', body: JSON.stringify(config) });
        configData = config;
    } catch (e) { showError('Failed to save config: ' + e.message); }
}

function resetConfig() {
    configData = {};
    activeTokens = TOKENS.map(t => t.symbol);
    activeDexes = DEXES.map(d => d.name);
    renderConfigForm();
    renderTokenBadges();
    renderDEXBadges();
}

async function checkBalance() {
    const display = document.getElementById('balance-display');
    if (!display) return;
    try {
        const data = await apiFetch('/api/contract/balance');
        display.innerHTML = `<div class="fee-card" style="display:inline-block">
            <div class="fee-label">Contract Balance</div>
            <div class="fee-value">${data ? formatUSD(data.balance || 0) : 'N/A'}</div>
            <div class="fee-unit">${data?.eth ? data.eth + ' ETH' : ''}</div>
        </div>`;
    } catch (e) {
        display.innerHTML = `<div style="color:var(--red);font-size:0.88rem;">Error: ${e.message}</div>`;
    }
}

async function withdrawProfits() {
    try {
        const data = await apiFetch('/api/contract/withdraw', { method: 'POST' });
        if (data && data.tx_hash) showError('Withdrawal submitted. TX: ' + data.tx_hash);
        else if (data && data.error) showError('Withdrawal error: ' + data.error);
    } catch (e) { showError('Withdrawal failed: ' + e.message); }
}

function setupPageNav() {
    document.querySelectorAll('.navbar-item').forEach(item => {
        item.addEventListener('click', (e) => {
            e.preventDefault();
            e.stopPropagation();
            const page = item.dataset.page;
            document.querySelectorAll('.navbar-item').forEach(i => i.classList.remove('active'));
            item.classList.add('active');
            document.querySelectorAll('.page').forEach(p => p.classList.remove('active'));
            const target = document.getElementById('page-' + page);
            if (target) target.classList.add('active');
        });
    });
}

document.addEventListener('DOMContentLoaded', checkAuth);
