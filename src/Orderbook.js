import './Orderbook.css';
import exchange from './Socket';
import { useState } from 'react';

function App() {
  let [depths, updateDepths] = useState({ asks: [], bids: [] });
  let [tradePrice, updateTradePrice] = useState(0);
  let [spread, updateSpread] = useState(0);

  exchange.ondepthschange = depths => {
    let asks = [];
    let bids = [];
    let bid_vol_max = 0;
    let ask_vol_max = 0;

    for (let price in depths.asks) {
      asks.push({
        price: (price / 100).toFixed(2),
        volume: depths.asks[price],
        side: 'ask'
      });

      if (depths.asks[price] > ask_vol_max) {
        ask_vol_max = depths.asks[price];
      }
    }
    asks.sort((a, b) => b.price - a.price);
    asks.forEach(e => e.width = parseInt(e.volume * 100 / ask_vol_max));

    for (let price in depths.bids) {
      bids.push({
        price: (price / 100).toFixed(2),
        volume: depths.bids[price],
        side: 'bid',
      });

      if (depths.bids[price] > bid_vol_max) {
        bid_vol_max = depths.bids[price];
      }

    }
    bids.sort((a, b) => b.price - a.price);
    bids.forEach(e => e.width = parseInt(e.volume * 100 / bid_vol_max));

    if (asks.length > 0 && bids.length > 0) {
      let spr = asks[asks.length - 1].price - bids[0].price;
      updateSpread(spr.toFixed(2));
    }


    updateDepths({ bids, asks });
  }

  exchange.ontrade = trade => {
    updateTradePrice((trade.price_cents / 100).toFixed(2));
  }

  return (
      <div className="order-book">
        {
          depths.asks.map(e => {
            return (
              <div className="row ask-row" key={e.price}>
                <div className="background-bar" style={{ width: e.width + '%' }}></div>
                <div className="numbers">
                  <div className='price'>${e.price}</div>
                  <div className='volume'>{e.volume}</div>
                </div>
              </div>
            );
          })
        }
        <div className="row" style={{ height: '30px', borderTop: '1px solid #464646', borderBottom: '1px solid #464646' }}>
          <div className="numbers" style={{ fontSize: '10pt' }}>
            <div style={{ textAlign: 'start' }}>${tradePrice}</div>
            <div style={{ textAlign: 'right' }}>{spread}</div>
          </div>
        </div>
        {
          depths.bids.map(e => {
            return (
              <div className="row bid-row" key={e.price}>
                <div className="background-bar" style={{ width: e.width + '%' }}></div>
                <div className="numbers">
                  <div className="price">${e.price}</div>
                  <div className='volume'>{e.volume}</div>
                </div>
              </div>
            );
          })
        }
      </div>
  );
}

export default App;
