pipeline {
    agent any
    stages {
        stage('Stage 1') {
            steps{
                tool name: 'sonar', type: 'hudson.plugins.sonar.SonarRunnerInstallation'
                withSonarQubeEnv('SonarCloud') {
                echo "hello";
            }
            }
        }
    }
}