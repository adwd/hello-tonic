import './App.css';
import { ClientStreamingHello } from './components/client-streaming';
import { OneofHello } from './components/oneof-hello';
import { SayHello } from './components/say-hello';
import { StreamingHello } from './components/streaming-hello';

function App() {
  return (
    <div className="App">
      <SayHello />
      <StreamingHello />
      <ClientStreamingHello />
      <OneofHello />
    </div>
  );
}

export default App;
