<script>
  

	
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
  
	setInterval(addSnowflake, 300);
	
	</script>
  
  <style>

  

  
	.content {
	  background-color: #4d4d4d; 
	  padding: 20px;
	  margin: 20px auto;
	  max-width: 600px; 
	}
  


  </style>
  
  <main>
	<h1>Advent of Code Solutions</h1>
	<div class="content">
	  <p>This repo contains the different solutions to the Advent of Code.</p>

	</div>
  </main>