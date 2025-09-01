// Image Modal System
// Handles expandable image display with smooth transitions

function openModal(image) {
    var modal = document.getElementById('imageModal');
    var modalImg = document.getElementById('modalImage');

    modalImg.src = image.src;

    modal.classList.remove('hidden');
    modal.classList.add('flex');
    modal.classList.add('opacity-0');

    // Force a reflow to ensure the opacity class is applied
    void modal.offsetWidth;

    modal.classList.remove('opacity-0');
    modal.classList.add('opacity-100');
}

function closeModal() {
    var modal = document.getElementById('imageModal');

    modal.classList.add('opacity-0');
    modal.classList.remove('opacity-100');

    setTimeout(function () {
        modal.classList.add('hidden');
        modal.classList.remove('flex');
    }, 300); // Match the duration of the transition
}

// Initialize modal event listeners when page loads
document.addEventListener('DOMContentLoaded', function () {
    // Add click listeners to all images with cursor-pointer class (modal trigger images)
    var modalImages = document.querySelectorAll('img.cursor-pointer');
    modalImages.forEach(function(img) {
        img.addEventListener('click', function() {
            openModal(this);
        });
    });

    // Add click listener to close button
    var closeButton = document.querySelector('#imageModal .cursor-pointer');
    if (closeButton) {
        closeButton.addEventListener('click', function() {
            closeModal();
        });
    }

    // Close the modal when clicking outside the image
    window.addEventListener('click', function (event) {
        var modal = document.getElementById('imageModal');
        
        if (event.target === modal) {
            closeModal();
        }
    });
});