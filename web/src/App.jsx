import { createSignal, onMount, For, Show } from 'solid-js';
import './index.css';
import gameplayImg from './assets/gameplay.png';

function App() {
  const [release, setRelease] = createSignal(null);
  const [loading, setLoading] = createSignal(true);

  onMount(async () => {
    try {
      const response = await fetch('https://api.github.com/repos/JanezSedeljsak/dino-ball/releases/latest');
      const data = await response.json();
      setRelease(data);
    } catch (error) {
      console.error('Error fetching release:', error);
    } finally {
      setLoading(false);
    }
  });

  const getUrl = (pattern) => {
    if (!release()) return '#';
    const asset = release().assets.find(a => a.name.toLowerCase().includes(pattern.toLowerCase()));
    return asset ? asset.browser_download_url : '#';
  };

  return (
    <div class="container">
      <header>
        <h1>Dino Volley</h1>
        <p class="subtitle">A 2D dinosaur volleyball game built with Rust.</p>
      </header>

      <div class="screenshot-container">
        <img src={gameplayImg} alt="Dino Volley Gameplay" />
      </div>

      <Show when={!loading()} fallback={<div class="loading">Loading latest version...</div>}>
        <div class="download-grid">
          <a href={getUrl('.deb')} class="download-card">
            <span>Linux</span>
            <span>.deb package</span>
          </a>
          <a href={getUrl('macos.zip')} class="download-card">
            <span>macOS</span>
            <span>.zip bundle</span>
          </a>
          <a href={getUrl('windows.zip')} class="download-card">
            <span>Windows</span>
            <span>.zip bundle</span>
          </a>
        </div>
        <p class="subtitle" style="margin-top: 1rem; font-size: 0.8rem;">
          Latest Version: {release()?.tag_name || 'v0.1.0'}
        </p>
      </Show>

      <footer style="margin-top: 4rem; color: #444; font-size: 0.8rem;">
        Made for my nephews :)
      </footer>
    </div>
  );
}

export default App;
