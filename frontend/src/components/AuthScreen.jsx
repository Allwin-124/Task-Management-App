import React, { useState } from 'react';
import { api } from '../services/api';

export function AuthScreen({ onAuthSuccess, onError, onSuccess }) {
  const [step, setStep] = useState('login'); // 'login' or '2fa'
  const [email, setEmail] = useState('admin@example.com');
  const [password, setPassword] = useState('Password123!');
  const [challengeId, setChallengeId] = useState('');
  const [code, setCode] = useState('');
  const [loading, setLoading] = useState(false);

  const handleLoginSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    try {
      const res = await api.login(email, password);
      setChallengeId(res.login_challenge_id);
      setStep('2fa');
      onSuccess(`Login challenge created! Check email logs or click 'Fetch Code'.`);
    } catch (err) {
      onError(err.message || 'Login failed');
    } finally {
      setLoading(false);
    }
  };

  const handleFetchLatestCode = async () => {
    try {
      const log = await api.getLatestEmailLog();
      if (log.code) {
        setCode(log.code);
        if (log.login_challenge_id) {
          setChallengeId(log.login_challenge_id);
        }
        onSuccess(`Auto-filled 2FA code (${log.code}) from dev email log.`);
      }
    } catch (err) {
      onError('Failed to fetch latest 2FA code from dev logs.');
    }
  };

  const handleVerify2FASubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    try {
      const tokenRes = await api.verify2FA(challengeId, code);
      const token = tokenRes.access_token;
      
      // Determine user role from email (or claims)
      const role = email.toLowerCase().includes('admin') ? 'admin' : 'staff';
      onAuthSuccess({ email, role, token });
      onSuccess(`Successfully authenticated via 2FA! Logged in as ${role.toUpperCase()}.`);
    } catch (err) {
      onError(err.message || '2FA verification failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ maxWidth: '440px', margin: '40px auto' }}>
      <div className="card">
        <div className="card-title">
          {step === 'login' ? '🔑 User Login' : '🔐 Two-Factor Authentication'}
        </div>

        {step === 'login' ? (
          <form onSubmit={handleLoginSubmit}>
            <div className="quick-fill-bar">
              <button
                type="button"
                className="btn btn-secondary btn-sm"
                onClick={() => {
                  setEmail('admin@example.com');
                  setPassword('Password123!');
                }}
              >
                Fill Admin
              </button>
              <button
                type="button"
                className="btn btn-secondary btn-sm"
                onClick={() => {
                  setEmail('jamesbond@example.com');
                  setPassword('Password123!');
                }}
              >
                Fill James Bond
              </button>
            </div>

            <div className="form-group">
              <label>Email Address</label>
              <input
                type="email"
                className="form-control"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder="admin@example.com"
                required
              />
            </div>

            <div className="form-group">
              <label>Password</label>
              <input
                type="password"
                className="form-control"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="••••••••"
                required
              />
            </div>

            <button
              type="submit"
              className="btn btn-primary"
              style={{ width: '100%', marginTop: '8px' }}
              disabled={loading}
            >
              {loading ? 'Authenticating...' : 'Continue to 2FA ➔'}
            </button>
          </form>
        ) : (
          <form onSubmit={handleVerify2FASubmit}>
            <p style={{ fontSize: '0.85rem', color: 'var(--text-secondary)', marginBottom: '16px' }}>
              Enter the 6-digit verification code sent to <strong>{email}</strong>.
            </p>

            <div className="quick-fill-bar">
              <button
                type="button"
                className="btn btn-seed btn-sm"
                onClick={handleFetchLatestCode}
              >
                📥 Fetch Code from Dev Log
              </button>
            </div>

            <div className="form-group">
              <label>Challenge ID</label>
              <input
                type="text"
                className="form-control"
                value={challengeId}
                onChange={(e) => setChallengeId(e.target.value)}
                readOnly
                style={{ opacity: 0.7 }}
              />
            </div>

            <div className="form-group">
              <label>6-Digit 2FA Code</label>
              <input
                type="text"
                className="form-control"
                value={code}
                onChange={(e) => setCode(e.target.value)}
                placeholder="e.g. 123456"
                maxLength={6}
                required
              />
            </div>

            <div style={{ display: 'flex', gap: '10px', marginTop: '16px' }}>
              <button
                type="button"
                className="btn btn-secondary"
                onClick={() => setStep('login')}
              >
                Back
              </button>
              <button
                type="submit"
                className="btn btn-primary"
                style={{ flex: 1 }}
                disabled={loading}
              >
                {loading ? 'Verifying...' : 'Verify Code & Login'}
              </button>
            </div>
          </form>
        )}
      </div>
    </div>
  );
}
