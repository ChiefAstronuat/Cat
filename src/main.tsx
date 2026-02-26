import React from 'react';
import ReactDOM from 'react-dom/client';
import OverlayCat from './components/OverlayCat';
import DebugHUD from './components/DebugHUD';
import './styles/globals.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <div className="app-container">
      <DebugHUD />
      <OverlayCat />
    </div>
  </React.StrictMode>
);
