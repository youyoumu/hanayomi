/* @refresh reload */
import { render } from "solid-js/web";

function App() {
	return "Hello world";
}

export default App;

const root = document.getElementById("root");

render(() => <App />, root!);
