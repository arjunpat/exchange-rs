import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Orderbook from './Orderbook';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <div style={{padding: '20px 200px', width: '800px'}}>
      <Orderbook />
    </div>
  </React.StrictMode>
);

