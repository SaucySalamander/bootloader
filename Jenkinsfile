pipeline {
    agent any
    tools {
        scannerHome 'sonar'
    }
    stages {
        stage('Stage 1') {
            steps {
                withSonarQubeEnv('SonarCloud') { // If you have configured more than one global server connection, you can specify its name
                    sh "${scannerHome}/bin/sonar-scanner"
                }
            }
        }
    }
}