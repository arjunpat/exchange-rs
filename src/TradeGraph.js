import exchange from './Socket';

function TradeGraph() {
  let trades = [];
  exchange.addTradeCallback(trade => {
    trades.push(trade);
  });

  return (
    <div>
      
    </div>
  )
}

export default TradeGraph;
