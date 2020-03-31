var myApp = angular.module('myApp', ['ngRoute'])

.controller('MainController', function($scope, $route, $routeParams, $location) {
    $scope.$route = $route;
    $scope.$location = $location;
    $scope.$routeParams = $routeParams;
})

.controller('JobsController', function($scope, $routeParams, $http) {
    $scope.name = 'JobsController';
    $scope.params = $routeParams;

    $scope.active_page = "jobs";
})

.controller('MachinesController', function($scope, $routeParams, $http) {
    $scope.name = 'MachinesController';
    $scope.params = $routeParams;

    $scope.active_page = "machines";
})

.controller('UsersController', function($scope, $routeParams, $http) {
    $scope.name = 'UsersController';
    $scope.params = $routeParams;

    $scope.active_page = "users";

    $http.get("/api/user/list")
        .then(function (response) {
            $scope.users = response.data;
        });
})

.controller('LogController', function($scope, $routeParams, $http) {
    $scope.name = 'LogController';
    $scope.params = $routeParams;

    $scope.active_page = "log";
})

.config(function($routeProvider, $locationProvider) {
   $routeProvider
   .when('/jobs/', {
        templateUrl: 'jobs.html',
        controller: 'JobsController'
    })
    .when('/machines/', {
        templateUrl: 'machines.html',
        controller: 'MachinesController'
    })
   .when('/users/', {
       templateUrl: 'users.html',
       controller: 'UsersController'
   })
   .when('/log/', {
        templateUrl: 'log.html',
        controller: 'LogController'
    });

   $locationProvider.html5Mode(true);
});
