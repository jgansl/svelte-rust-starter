<script>
	import {Content} from 'carbon-components-svelte';
	
	export let bindings;

	// let {Car,color} = bindings; // destructure for easier access
	// console.log(Car,color);
	let {Car,color} = bindings; // destructure for easier access

	let c = Car.new();
	c.number = 5;
	c.color = 775577;
	console.log(c);

	let c2 = color(c,557755);
	console.log(c2.number,c2.color);
	
	let c3 = c2.duplicate();
	c3.change_number(7);
	console.log(c3.number);
	console.log(c3.number,c3.color);
	// console.log(c3.get_id());

	console.log(c2,c3);
</script>

<main>
	<Content>
		<h1>{bindings.add(11,7)}</h1>
	</Content>
</main>
