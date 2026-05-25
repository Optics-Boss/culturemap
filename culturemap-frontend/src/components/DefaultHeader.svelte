<script>
  import { page } from '$app/stores';
</script>

<nav class="container">
  <a 
      class:active={$page.url.pathname === '/list'} 
      href="/list"
  >
      List
  </a>
  <a 
    href="/map"
    class:active={$page.url.pathname === '/map'} 
  >
    Map
  </a>
</nav>

<style>
  .container {
    display: flex;
    gap: 0.625%;
  }

  a {
    flex: 1;
    text-decoration: none;
    outline-color: transparent;
    text-align: center;
    line-height: 3;
    color: black;
    font-size: 20px;
    font-weight: 600;
  }

  a:link,
  a:visited,
  a:focus {
    background: #669989;
    color: black;
  }

  a:hover {
    background: #669099;
  }

  a:active {
    background: #66996f;
    color: white;
  }

  a.active {
    background: #66996f;
    color: white;
  }
</style>
