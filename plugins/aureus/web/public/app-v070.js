/**
 * Git Flow Master v0.7.0 - Premium UI Features
 * Theme Toggle, Sidebar Management, Status API Integration
 */

// ============================================================================
// THEME MANAGER
// ============================================================================

const ThemeManager = {
  STORAGE_KEY: 'git-flow-master-theme',

  init() {
    // Load saved theme or use system preference
    const savedTheme = localStorage.getItem(this.STORAGE_KEY);
    if (savedTheme) {
      this.setTheme(savedTheme);
    } else {
      // Auto-detect system preference
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      this.setTheme(prefersDark ? 'dark' : 'light');
    }

    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
      if (!localStorage.getItem(this.STORAGE_KEY)) {
        this.setTheme(e.matches ? 'dark' : 'light');
      }
    });

    // Setup toggle button
    const toggleBtn = document.getElementById('themeToggle');
    if (toggleBtn) {
      toggleBtn.addEventListener('click', () => this.toggle());
    }
  },

  toggle() {
    const current = this.getTheme();
    const newTheme = current === 'dark' ? 'light' : 'dark';
    this.setTheme(newTheme);
  },

  setTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem(this.STORAGE_KEY, theme);

    // Update icons
    const sunIcon = document.querySelector('.sun-icon');
    const moonIcon = document.querySelector('.moon-icon');

    if (theme === 'dark') {
      sunIcon.style.display = 'block';
      moonIcon.style.display = 'none';
    } else {
      sunIcon.style.display = 'none';
      moonIcon.style.display = 'block';
    }
  },

  getTheme() {
    return document.documentElement.getAttribute('data-theme') || 'dark';
  }
};

// ============================================================================
// SIDEBAR MANAGER
// ============================================================================

const SidebarManager = {
  backdropHandler: null,
  keydownHandler: null,

  init() {
    const settingsBtn = document.getElementById('settingsBtn');
    const closeSidebarBtn = document.getElementById('closeSidebar');
    const sidebar = document.getElementById('sidebar');

    if (settingsBtn) {
      settingsBtn.addEventListener('click', () => this.open());
    }

    if (closeSidebarBtn) {
      closeSidebarBtn.addEventListener('click', () => this.close());
    }

    // Close on backdrop click - store reference for cleanup
    this.backdropHandler = (e) => {
      const sidebar = document.getElementById('sidebar');
      const settingsBtn = document.getElementById('settingsBtn');
      if (sidebar && sidebar.classList.contains('open')) {
        if (!sidebar.contains(e.target) && settingsBtn && !settingsBtn.contains(e.target)) {
          this.close();
        }
      }
    };
    document.addEventListener('click', this.backdropHandler);

    // Close on Escape key - exclude inputs
    this.keydownHandler = (e) => {
      const sidebar = document.getElementById('sidebar');
      if (e.key === 'Escape' &&
          sidebar &&
          sidebar.classList.contains('open') &&
          !e.target.matches('input, textarea, select')) {
        e.preventDefault();
        this.close();
      }
    };
    document.addEventListener('keydown', this.keydownHandler);
  },

  open() {
    const sidebar = document.getElementById('sidebar');
    const settingsBtn = document.getElementById('settingsBtn');

    if (sidebar) sidebar.classList.add('open');
    if (settingsBtn) settingsBtn.classList.add('active');
  },

  close() {
    const sidebar = document.getElementById('sidebar');
    const settingsBtn = document.getElementById('settingsBtn');

    if (sidebar) sidebar.classList.remove('open');
    if (settingsBtn) settingsBtn.classList.remove('active');
  },

  toggle() {
    const sidebar = document.getElementById('sidebar');
    if (sidebar && sidebar.classList.contains('open')) {
      this.close();
    } else {
      this.open();
    }
  },

  destroy() {
    // Cleanup event listeners to prevent memory leak
    if (this.backdropHandler) {
      document.removeEventListener('click', this.backdropHandler);
      this.backdropHandler = null;
    }
    if (this.keydownHandler) {
      document.removeEventListener('keydown', this.keydownHandler);
      this.keydownHandler = null;
    }
  }
};

// ============================================================================
// STATUS MANAGER
// ============================================================================

const StatusManager = {
  statusCheckInterval: null,
  CHECK_INTERVAL: 30000, // 30 seconds

  async init() {
    // Initial status check
    await this.checkStatus();

    // Setup auto-refresh
    this.statusCheckInterval = setInterval(() => {
      this.checkStatus();
    }, this.CHECK_INTERVAL);
  },

  async checkStatus() {
    try {
      const response = await fetch(`${API_BASE}/status`);
      const data = await response.json();

      this.updateStatusIndicator(data.status);
      this.updateStatistics(data.statistics || {});
    } catch (error) {
      console.error('Status check failed:', error);
      this.updateStatusIndicator('offline');
    }
  },

  updateStatusIndicator(status) {
    const statusDot = document.getElementById('statusDot');

    if (!statusDot) return;

    if (status === 'online') {
      statusDot.classList.add('connected');
    } else {
      statusDot.classList.remove('connected');
    }
  },

  updateStatistics(stats) {
    // Update stat cards
    const updateElement = (id, value) => {
      const el = document.getElementById(id);
      if (el) el.textContent = value;
    };

    updateElement('repoCount', stats.repositories || 0);
    updateElement('hooksCount', stats.hooksInstalled || 0);
    updateElement('commitsCount', stats.recentCommits || 0);

    // Format uptime
    const uptime = stats.uptime || 0;
    updateElement('uptimeValue', this.formatUptime(uptime));

    // Update sidebar stats
    updateElement('statTotalRepos', stats.repositories || 0);
    updateElement('statTotalHooks', stats.hooksInstalled || 0);
    updateElement('statCompliance', '0%'); // Will be calculated later
  },

  formatUptime(seconds) {
    if (seconds < 60) {
      return `${seconds}s`;
    } else if (seconds < 3600) {
      const minutes = Math.floor(seconds / 60);
      const secs = seconds % 60;
      return secs > 0 ? `${minutes}m ${secs}s` : `${minutes}m`;
    } else {
      const hours = Math.floor(seconds / 3600);
      const minutes = Math.floor((seconds % 3600) / 60);
      return minutes > 0 ? `${hours}h ${minutes}m` : `${hours}h`;
    }
  },

  destroy() {
    if (this.statusCheckInterval) {
      clearInterval(this.statusCheckInterval);
    }
  }
};

// ============================================================================
// VALIDATOR
// ============================================================================

const Validator = {
  sanitizeString(str) {
    if (typeof str !== 'string') return '';
    const div = document.createElement('div');
    div.textContent = str;
    return div.innerHTML;
  },

  validateNumber(num) {
    const n = parseInt(num, 10);
    if (!Number.isFinite(n) || n < 0) return 0;
    return n;
  },

  validateRepoName(name) {
    if (typeof name !== 'string') return '';
    if (name.length > 255) return name.substring(0, 255);
    return this.sanitizeString(name);
  }
};

// ============================================================================
// GLOBAL FUNCTIONS (for HTML onclick handlers)
// ============================================================================

// Quick Actions
async function scanAllRepos() {
  try {
    const response = await fetch(`${API_BASE}/scan-repos`, {
      method: 'POST'
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }

    const data = await response.json();

    if (data.success) {
      const count = Validator.validateNumber(data.count);
      Toast.success(`Scanned ${count} repositories successfully!`);

      // Refresh state after scan
      if (typeof refreshState === 'function') {
        await refreshState();
      }
    } else {
      Toast.error('Failed to scan repositories');
    }
  } catch (error) {
    console.error('Scan error:', error);
    Toast.error('Error scanning repositories');
  }
}

async function loadCurrentRepo() {
  try {
    const response = await fetch(`${API_BASE}/load-current-repo`, {
      method: 'POST'
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }

    const data = await response.json();

    if (data.success) {
      const repo = Validator.validateRepoName(data.repo);
      Toast.success(`Loaded repository: ${repo}`);

      if (typeof refreshState === 'function') {
        await refreshState();
      }
    } else {
      const errorMsg = Validator.sanitizeString(data.error || 'Failed to load repository');
      Toast.error(errorMsg);
    }
  } catch (error) {
    console.error('Load repo error:', error);
    Toast.error('Error loading repository');
  }
}

async function refreshState() {
  try {
    // Trigger Alpine data refresh
    if (window.Alpine) {
      const app = Alpine.store('gitFlowApp');
      if (app && typeof app.loadState === 'function') {
        await app.loadState();
      }
    }

    // Refresh status
    await StatusManager.checkStatus();
  } catch (error) {
    console.error('Refresh error:', error);
  }
}

async function saveConfig() {
  try {
    const projectNameInput = document.getElementById('configProjectName');
    const commitTypeInput = document.getElementById('configCommitType');

    const projectName = Validator.sanitizeString(projectNameInput?.value || '');
    const commitType = Validator.sanitizeString(commitTypeInput?.value || 'PATCH');

    const response = await fetch(`${API_BASE}/config`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        projectName,
        defaultCommitType: commitType
      })
    });

    if (response.ok) {
      Toast.success('Configuration saved successfully!');
      SidebarManager.close();
    } else {
      Toast.error('Failed to save configuration');
    }
  } catch (error) {
    console.error('Save config error:', error);
    Toast.error('Error saving configuration');
  }
}

// ============================================================================
// INITIALIZATION
// ============================================================================

document.addEventListener('DOMContentLoaded', () => {
  // Initialize managers
  ThemeManager.init();
  SidebarManager.init();
  StatusManager.init();
});

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
  StatusManager.destroy();
  SidebarManager.destroy(); // Cleanup memory leak
});
