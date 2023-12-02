<script>
import Home from './Home.svelte';
import Snowflake from './Snowflake.svelte';
import Navigation from './Navigation.svelte';
import { Router, Route } from "svelte-routing";
import SolutionsShower from './SolutionsShower.svelte';	
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
	main {
	  text-align: center; 
	  height: 120vh;
    margin: 0;
    padding: 0;
    background-color: #333333; 
    color: #FFD700; 
    font-family: Arial, sans-serif;
	}

  
  </style>
	  <main>
  <Router>

	  <Navigation />
	  <Route path="/" component={Home} />
	  <!-- <Route path="/sabout" component={SolutionsShower} /> -->


  </Router>
  {#each snowflakes as snowflake}
	  <Snowflake size={snowflake.size} left={snowflake.left} duration={snowflake.duration} />
  {/each}
  </main>