<script>
	import Snowflake from './Snowflake.svelte';
  
	 let snowflakes = [];
 
	function addSnowflake() {
    snowflakes = [
      ...snowflakes,
      {
        size: Math.random() * 20 + 10,
        left: Math.random() * 100, 
        duration: Math.random() * 5 + 3
      }
    ];

    if (snowflakes.length > 200) {
      snowflakes.shift();
    }
  }
  
	setInterval(addSnowflake, 300); // Add a snowflake every 100ms
	
	</script>
  
  <style>

  
	main {
	  text-align: center; /* Center text in the main tag */
	  height: 120vh;
    margin: 0;
    padding: 0;
    background-color: #333333; /* Dark grey background */
    color: #FFD700; /* Yellow text */
    font-family: Arial, sans-serif;
	}
  
	.content {
	  background-color: #4d4d4d; /* Lighter dark grey background for the content */
	  padding: 20px;
	  margin: 20px auto;
	  max-width: 600px; /* Adjust width as needed */
	}
  
	h1 {
	}
  
	/* Additional styling as needed */
  </style>
  
  <main>
	<h1>Advent of Code Solutions</h1>
	<div class="content">
	  <p>This repo contains the different solutions to the Advent of Code.</p>
	  <!-- More content can be added here -->
	</div>
  
  {#each snowflakes as snowflake}
	  <Snowflake size={snowflake.size} left={snowflake.left} duration={snowflake.duration} />
  {/each}
  </main>