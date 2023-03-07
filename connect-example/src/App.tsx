import './App.css';
import { OneofHello } from './components/oneof-hello';
import { SayHello } from './components/say-hello';
import { StreamingHello } from './components/streaming-hello';

function App() {
  return (
    <div className="App">
      <SayHello />
      <StreamingHello />
      <OneofHello />
    </div>
  );
}

export default App;
