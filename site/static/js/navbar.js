var url = window.location.href;
url = '/' + url.substr(url.lastIndexOf('/') + 1);
$(document).ready(function() {
    $('nav').find('a[href="' + url + '"]').addClass('currentlink');
});
