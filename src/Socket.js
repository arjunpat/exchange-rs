const WEBSOCKET_URL = 'ws://localhost:8080/ws';

class Socket {
  constructor(url) {
    this.ws = new WebSocket(url);

    this.ws.onmessage = msg => {
      const data = JSON.parse(msg.data);
      if (data.Depths) {
        if (this.ondepthschange) {
          this.ondepthschange(data.Depths);
        }
      } else if (data.Trade) {
        if (this.ontrade) {
          this.ontrade(data.Trade);
        }
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
          if (Math.random() > .5) {
            let size = Math.floor(Math.random() * 20);
            let price = Math.floor(Math.random() * 30) + 100;
            this.buy("AAPL", size, price);
          } else {
            let size = Math.floor(Math.random() * 20);
            let price = Math.floor(Math.random() * 30) + 120;
            this.sell("AAPL", size, price);
          }
        }, 10);
    }
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


export default socket;
