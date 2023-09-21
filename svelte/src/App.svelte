<script>
  import { Content, FileUploader } from "carbon-components-svelte";

  export let bindings;
  let files = []; // files handles will be added in this
  let content = ""; // for us to see the contents
  let reader = new FileReader(); // for reading the file contents

  reader.addEventListener(
    "load",
    () => {
      content = reader.result; // set the contents in ui, so we can see it
    },
    false
  );

  // this will be used as callback to file uploader
  let add_handler = (e) => {
    reader.readAsText(e.detail[0]);
  };

  // // let {Car,color} = bindings; // destructure for easier access
  // // console.log(Car,color);
  // let {Car,color} = bindings; // destructure for easier access

  // let c = Car.new();
  // c.number = 5;
  // c.color = 775577;
  // console.log(c);

  // let c2 = color(c,557755);
  // console.log(c2.number,c2.color);

  // let c3 = c2.duplicate();
  // c3.change_number(7);
  // console.log(c3.number);
  // console.log(c3.number,c3.color);
  // console.log(c3.get_id());

  // console.log(c2,c3);
</script>

<!-- <main>
	<Content>
		<h1>{bindings.add(11,7)}</h1>
	</Content>
</main> -->

<Content>
  <h1>Connecting rust to Svelte Through wasm !</h1>

  <br />

  <div>
    <FileUploader
      labelTitle="Upload file"
      buttonLabel="Add file"
      labelDescription="Only txt files are accepted."
      accept={[".txt"]}
      bind:files
      status="complete"
      on:add={add_handler}
    />
  </div>

  <br />

  <h3>
    File Contents Are :<br />
    {content}
  </h3>
</Content>
