pipeline {
    agent any
    tools {
        SonarRunnerInstallation 'sonar'
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