// Disable CSS loading
const styleTags = document.querySelectorAll('link[rel="stylesheet"]');
styleTags.forEach(tag => tag.disabled = true);

// Disable image loading
const images = document.querySelectorAll('img');
images.forEach(img => img.style.display = 'none'); // Alternative: img.src = '';

