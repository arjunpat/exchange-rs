const WEBSOCKET_URL = 'ws://localhost:8080/ws';

class Socket {
  constructor(url) {
    this.ws = new WebSocket(url);
    this.tradeCallbacks = [];
    this.price_cents = 0;

    this.ws.onmessage = msg => {
      const data = JSON.parse(msg.data);
      if (data.Depths) {
        if (this.ondepthschange) {
          this.ondepthschange(data.Depths);
        }
      } else if (data.Trade) {
        this.ontrade(data.Trade);
      }
    }

    this.ws.onopen = () => {
        for (let price = 100; price < 125; price++) {
          this.buy("AAPL", 30, price);
        }

        for (let price = 126; price < 150; price++) {
          this.sell("AAPL", 30, price);
        }

        setInterval(() => {
          let size = validGaussianRandom(20, 10);
          if (Math.random() > .2) {
            let price = validGaussianRandom(this.price_cents, 5);
            this.buy("AAPL", size, price);
          } else {
            let price = validGaussianRandom(this.price_cents, 5);
            this.sell("AAPL", size, price);
          }
        }, 100);
    }
  }

  ontrade(trade) {
    this.tradeCallbacks.forEach(e => e(trade));
    this.price_cents = trade.price_cents;
  }

  addTradeCallback(func) {
    this.tradeCallbacks.push(func);
  }

  generateOrder(security, size, price_cents, buy) {
    return JSON.stringify({
      Order: {
        security, size, price_cents, buy
      }
    });
  }

  buy(security, size, price_cents) {
    this.ws.send(this.generateOrder(security, size, price_cents, true))
  }

  sell(security, size, price_cents) {
    this.ws.send(this.generateOrder(security, size, price_cents, false));
  }
}

const socket = new Socket(WEBSOCKET_URL);

function validGaussianRandom(mean = 0, stdev = 1) {
  return Math.max(Math.round(gaussianRandom(mean, stdev)), 0);
}

function gaussianRandom(mean = 0, stdev = 1) {
    const u = 1 - Math.random(); // Converting [0,1) to (0,1]
    const v = Math.random();
    const z = Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);
    // Transform to the desired mean and standard deviation:
    return z * stdev + mean;
}


export default socket;
