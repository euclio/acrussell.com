var url = window.location.pathname;
var first_slash = url.indexOf('/', 1);

var substring_length = first_slash > -1 ? first_slash : url.length;
url = url.substring(0, substring_length);

$(document).ready(function() {
    $('nav').find('a[href^="' + url + '"]').first().addClass('currentlink');
});
