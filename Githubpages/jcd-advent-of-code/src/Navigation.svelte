<script>
    import { Link } from "svelte-routing";
   </script>
   
   <style>
    nav {
      display: flex;
      justify-content: center;

      color: #FFD700; 
    font-family: Arial, sasans-serif;
    
    margin: 0 10px;
    color: #FFD700; 
    font-family: Arial, sans-serif;}
    .aSom {
      margin: 0 10px;
    color: #FFD700; 
    font-family: Arial, sans-serif;
    }
    .aSom:hover {

    }
   </style>
   
   <nav>
    <Link to="/" class="aSom">Home</Link>
    <Link to="/about" class="aSom">About</Link>
   </nav>
   