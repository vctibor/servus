

angular.module('myApp', ['ngRoute'])

 .controller('MainController', function($scope, $route, $routeParams, $location) {
     $scope.$route = $route;
     $scope.$location = $location;
     $scope.$routeParams = $routeParams;
 })

 .controller('UsersController', function($scope, $routeParams) {
     $scope.name = 'UsersController';
     $scope.params = $routeParams;
 })

.config(function($routeProvider, $locationProvider) {
    $routeProvider.when('/users/', {
        templateUrl: 'book.html',
        controller: 'UsersController'
    });

    $locationProvider.html5Mode(true);
});

