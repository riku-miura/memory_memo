class MemoryMemoApp {
    constructor() {
        this.currentUser = null;
        this.memos = { forever_memos: [], flush_memos: [] };
        this.apiBase = '/api';
        
        this.init();
    }
    
    init() {
        this.bindEvents();
        this.checkAuth();
    }
    
    bindEvents() {
        // Auth tab switching
        document.getElementById('login-tab').addEventListener('click', () => this.switchTab('login'));
        document.getElementById('register-tab').addEventListener('click', () => this.switchTab('register'));
        
        // Auth forms
        document.getElementById('login-form').addEventListener('submit', (e) => this.handleLogin(e));
        document.getElementById('register-form').addEventListener('submit', (e) => this.handleRegister(e));
        
        // Main app events
        document.getElementById('logout-btn').addEventListener('click', () => this.handleLogout());
        document.getElementById('memo-form').addEventListener('submit', (e) => this.handleCreateMemo(e));
        
        // Modal events
        document.getElementById('close-modal').addEventListener('click', () => this.hideModal());
        document.getElementById('cancel-edit').addEventListener('click', () => this.hideModal());
        document.getElementById('edit-memo-form').addEventListener('submit', (e) => this.handleEditMemo(e));
        
        // Close modal on backdrop click
        document.getElementById('edit-modal').addEventListener('click', (e) => {
            if (e.target.id === 'edit-modal') {
                this.hideModal();
            }
        });
    }
    
    switchTab(tab) {
        const loginTab = document.getElementById('login-tab');
        const registerTab = document.getElementById('register-tab');
        const loginForm = document.getElementById('login-form');
        const registerForm = document.getElementById('register-form');
        
        if (tab === 'login') {
            loginTab.classList.add('active');
            registerTab.classList.remove('active');
            loginForm.classList.remove('hidden');
            registerForm.classList.add('hidden');
        } else {
            registerTab.classList.add('active');
            loginTab.classList.remove('active');
            registerForm.classList.remove('hidden');
            loginForm.classList.add('hidden');
        }
        
        this.clearError();
    }
    
    async checkAuth() {
        // For simplicity, we'll just check if we can access the memos endpoint
        // In a real app, you'd have a dedicated auth check endpoint
        try {
            await this.loadMemos();
            this.showMainPage();
        } catch (error) {
            this.showAuthPage();
        }
    }
    
    showAuthPage() {
        document.getElementById('auth-page').classList.remove('hidden');
        document.getElementById('main-page').classList.add('hidden');
    }
    
    showMainPage() {
        document.getElementById('auth-page').classList.add('hidden');
        document.getElementById('main-page').classList.remove('hidden');
        this.loadMemos();
    }
    
    showLoading() {
        document.getElementById('loading-overlay').classList.remove('hidden');
    }
    
    hideLoading() {
        document.getElementById('loading-overlay').classList.add('hidden');
    }
    
    showError(message, elementId = 'auth-error') {
        const errorElement = document.getElementById(elementId);
        errorElement.textContent = message;
        errorElement.classList.add('show');
    }
    
    clearError(elementId = 'auth-error') {
        const errorElement = document.getElementById(elementId);
        errorElement.classList.remove('show');
        errorElement.textContent = '';
    }
    
    async handleLogin(e) {
        e.preventDefault();
        
        const username = document.getElementById('login-username').value.trim();
        const password = document.getElementById('login-password').value;
        
        if (!username || !password) {
            this.showError('ユーザー名とパスワードを入力してください。');
            return;
        }
        
        try {
            this.showLoading();
            this.clearError();
            
            const response = await fetch(`${this.apiBase}/auth/login`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'include',
                body: JSON.stringify({ username, password })
            });
            
            if (response.ok) {
                const data = await response.json();
                this.currentUser = data.username;
                document.getElementById('username-display').textContent = `@${this.currentUser}`;
                this.showMainPage();
            } else {
                const error = await response.json();
                this.showError(error.error || 'ログインに失敗しました。');
            }
        } catch (error) {
            this.showError('ネットワークエラーが発生しました。');
        } finally {
            this.hideLoading();
        }
    }
    
    async handleRegister(e) {
        e.preventDefault();
        
        const username = document.getElementById('register-username').value.trim();
        const password = document.getElementById('register-password').value;
        
        if (!username || !password) {
            this.showError('ユーザー名とパスワードを入力してください。');
            return;
        }
        
        if (username.length < 3) {
            this.showError('ユーザー名は3文字以上で入力してください。');
            return;
        }
        
        if (password.length < 8) {
            this.showError('パスワードは8文字以上で入力してください。');
            return;
        }
        
        try {
            this.showLoading();
            this.clearError();
            
            const response = await fetch(`${this.apiBase}/auth/register`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'include',
                body: JSON.stringify({ username, password })
            });
            
            if (response.ok) {
                // Auto-login after successful registration
                this.clearError();
                document.getElementById('register-form').reset();
                this.switchTab('login');
                
                // Show success message and auto-fill login form
                document.getElementById('login-username').value = username;
                this.showError('登録が完了しました。ログインしてください。', 'auth-error');
                setTimeout(() => this.clearError(), 3000);
            } else {
                const error = await response.json();
                this.showError(error.error || '登録に失敗しました。');
            }
        } catch (error) {
            this.showError('ネットワークエラーが発生しました。');
        } finally {
            this.hideLoading();
        }
    }
    
    async handleLogout() {
        try {
            this.showLoading();
            
            await fetch(`${this.apiBase}/auth/logout`, {
                method: 'POST',
                credentials: 'include'
            });
            
            this.currentUser = null;
            this.memos = { forever_memos: [], flush_memos: [] };
            
            // Clear forms
            document.getElementById('login-form').reset();
            document.getElementById('register-form').reset();
            document.getElementById('memo-form').reset();
            
            this.showAuthPage();
        } catch (error) {
            console.error('Logout error:', error);
            // Force logout even if API call fails
            this.showAuthPage();
        } finally {
            this.hideLoading();
        }
    }
    
    async loadMemos() {
        try {
            const response = await fetch(`${this.apiBase}/memos`, {
                credentials: 'include'
            });
            
            if (response.ok) {
                this.memos = await response.json();
                this.renderMemos();
            } else {
                throw new Error('Failed to load memos');
            }
        } catch (error) {
            console.error('Load memos error:', error);
            throw error;
        }
    }
    
    renderMemos() {
        this.renderForeverMemos();
        this.renderFlushMemos();
    }
    
    renderForeverMemos() {
        const container = document.getElementById('forever-memos');
        
        if (this.memos.forever_memos.length === 0) {
            container.innerHTML = '<div class="empty-state">永続メモはありません</div>';
            return;
        }
        
        container.innerHTML = this.memos.forever_memos.map(memo => `
            <div class="memo-card forever" data-id="${memo.id}">
                <div class="memo-content">${this.escapeHtml(memo.content)}</div>
                <div class="memo-meta">
                    <span>${this.formatDate(memo.created_at)}</span>
                    <span>永続</span>
                </div>
                <div class="memo-actions">
                    <button class="btn-edit" onclick="app.editMemo('${memo.id}', 'forever')">編集</button>
                    <button class="btn-delete" onclick="app.deleteMemo('${memo.id}', 'forever')">削除</button>
                </div>
            </div>
        `).join('');
    }
    
    renderFlushMemos() {
        const container = document.getElementById('flush-memos');
        
        if (this.memos.flush_memos.length === 0) {
            container.innerHTML = '<div class="empty-state">24時間メモはありません</div>';
            return;
        }
        
        container.innerHTML = this.memos.flush_memos.map(memo => `
            <div class="memo-card flush" data-id="${memo.id}">
                <div class="memo-content">${this.escapeHtml(memo.content)}</div>
                <div class="memo-meta">
                    <span>${this.formatDate(memo.created_at)}</span>
                    <span class="memo-expires">期限: ${this.formatDate(memo.expires_at)}</span>
                </div>
                <div class="memo-actions">
                    <button class="btn-delete" onclick="app.deleteMemo('${memo.id}', 'flush')">削除</button>
                </div>
            </div>
        `).join('');
    }
    
    async handleCreateMemo(e) {
        e.preventDefault();
        
        const content = document.getElementById('memo-content').value.trim();
        const type = e.submitter.value; // 'forever' or 'flush'
        
        if (!content) {
            alert('メモの内容を入力してください。');
            return;
        }
        
        try {
            this.showLoading();
            
            const response = await fetch(`${this.apiBase}/memos/${type}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'include',
                body: JSON.stringify({ content })
            });
            
            if (response.ok) {
                document.getElementById('memo-form').reset();
                await this.loadMemos();
            } else {
                const error = await response.json();
                alert(error.error || 'メモの作成に失敗しました。');
            }
        } catch (error) {
            alert('ネットワークエラーが発生しました。');
        } finally {
            this.hideLoading();
        }
    }
    
    editMemo(memoId, type) {
        if (type !== 'forever') {
            alert('24時間メモは編集できません。');
            return;
        }
        
        const memo = this.memos.forever_memos.find(m => m.id === memoId);
        if (!memo) return;
        
        document.getElementById('edit-memo-content').value = memo.content;
        document.getElementById('edit-memo-form').dataset.memoId = memoId;
        this.showModal();
    }
    
    async handleEditMemo(e) {
        e.preventDefault();
        
        const content = document.getElementById('edit-memo-content').value.trim();
        const memoId = e.target.dataset.memoId;
        
        if (!content) {
            alert('メモの内容を入力してください。');
            return;
        }
        
        try {
            this.showLoading();
            
            const response = await fetch(`${this.apiBase}/memos/forever/${memoId}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'include',
                body: JSON.stringify({ content })
            });
            
            if (response.ok) {
                this.hideModal();
                await this.loadMemos();
            } else {
                const error = await response.json();
                alert(error.error || 'メモの更新に失敗しました。');
            }
        } catch (error) {
            alert('ネットワークエラーが発生しました。');
        } finally {
            this.hideLoading();
        }
    }
    
    async deleteMemo(memoId, type) {
        if (!confirm('このメモを削除しますか？')) {
            return;
        }
        
        try {
            this.showLoading();
            
            const response = await fetch(`${this.apiBase}/memos/${type}/${memoId}`, {
                method: 'DELETE',
                credentials: 'include'
            });
            
            if (response.ok) {
                await this.loadMemos();
            } else {
                const error = await response.json();
                alert(error.error || 'メモの削除に失敗しました。');
            }
        } catch (error) {
            alert('ネットワークエラーが発生しました。');
        } finally {
            this.hideLoading();
        }
    }
    
    showModal() {
        document.getElementById('edit-modal').classList.remove('hidden');
    }
    
    hideModal() {
        document.getElementById('edit-modal').classList.add('hidden');
        document.getElementById('edit-memo-form').reset();
        delete document.getElementById('edit-memo-form').dataset.memoId;
    }
    
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    formatDate(dateString) {
        const date = new Date(dateString);
        const now = new Date();
        const diffMs = now - date;
        const diffMins = Math.floor(diffMs / 60000);
        const diffHours = Math.floor(diffMs / 3600000);
        const diffDays = Math.floor(diffMs / 86400000);
        
        if (diffMins < 1) return 'たった今';
        if (diffMins < 60) return `${diffMins}分前`;
        if (diffHours < 24) return `${diffHours}時間前`;
        if (diffDays < 7) return `${diffDays}日前`;
        
        return date.toLocaleDateString('ja-JP', {
            year: 'numeric',
            month: 'short',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        });
    }
}

// Initialize app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.app = new MemoryMemoApp();
});

// Refresh memos every 30 seconds to show updated expiry times
setInterval(() => {
    if (window.app && window.app.currentUser) {
        window.app.loadMemos();
    }
}, 30000);