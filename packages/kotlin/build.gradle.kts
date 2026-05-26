import com.vanniktech.maven.publish.JavadocJar
import com.vanniktech.maven.publish.KotlinJvm
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

buildscript {
  dependencies {
    classpath("com.vanniktech:gradle-maven-publish-plugin:0.36.0")
  }
}

plugins {
  `java-library`
  kotlin("jvm") version "2.3.21"
  id("com.vanniktech.maven.publish") version "0.36.0"
  id("org.jlleitschuh.gradle.ktlint") version "13.1.0"
}

group = "dev.spikard"
version = "0.15.6-rc.5"

repositories {
  mavenCentral()
}

dependencies {
  api("net.java.dev.jna:jna:5.18.1")
  // Jackson is on the public surface because the alef-emitted Java records
  // include `@JsonProperty` annotations for serialization round-tripping.
  api("com.fasterxml.jackson.core:jackson-annotations:2.21")
  api("com.fasterxml.jackson.core:jackson-databind:2.21.3")
  api("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.21.3")
  // jspecify ships the `@Nullable` / `@NonNull` annotations referenced by the
  // alef-emitted Java facade; it must be on the api configuration so Kotlin
  // consumers see the annotations on cross-language types.
  api("org.jspecify:jspecify:1.0.0")
  implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.11.0")
  testImplementation("org.jetbrains.kotlin:kotlin-test:2.3.21")
  testImplementation("junit:junit:4.13.2")
}

java {
  sourceCompatibility = JavaVersion.VERSION_21
  targetCompatibility = JavaVersion.VERSION_21
}

// Include the alef-emitted Java facade (sibling package) so the Kotlin object
// can call into the JNA-loaded native bridge. The Kotlin backend places its
// generated files in a sub-package (`<group>.kt`) to avoid colliding with the
// Java facade that uses the canonical `<group>` package.
sourceSets {
  main {
    java {
      // Pull in the Java facade emitted by the alef Java backend so the
      // Kotlin module compiles against the same on-disk sources. The alef
      // Java backend writes to `packages/java/` (package-root layout), not
      // the Maven `src/main/java/` convention.
      srcDir("../java")
    }
  }
}

kotlin {
  compilerOptions {
    jvmTarget.set(JvmTarget.JVM_21)
  }
}

// ktlint configuration — see .editorconfig for details. We deliberately exclude
// the Java facade (which lives under `packages/java/`) and any build/generated
// directories: ktlint cannot lint pure-Java files, and the FFM/Panama bindings
// are kept in their own module.
ktlint {
  version.set("1.8.0")
  outputToConsole.set(true)
  ignoreFailures.set(false)
  filter {
    exclude { entry -> entry.file.toString().contains("/packages/java/") }
    exclude { entry -> entry.file.toString().endsWith("/Spikard.kt") }
    exclude("**/build/**")
    exclude("**/generated/**")
  }
}

// Gradle 9.x flags an output-overlap validation error between
// :ktlintKotlinScriptCheck / :ktlintMainSourceSetCheck and :compileKotlin.
// Declare the explicit dependency so Gradle accepts the task graph.
tasks.matching { it.name == "compileKotlin" }.configureEach {
  mustRunAfter("ktlintKotlinScriptCheck")
  mustRunAfter("ktlintMainSourceSetCheck")
}

// JNA needs the native lib on java.library.path; default to the workspace
// `target/release` cargo output. Override with `-Pnative.lib.path=<dir>`.
tasks.withType<Test>().configureEach {
  val libPath = (project.findProperty("native.lib.path") as String?) ?: "$rootDir/../../target/release"
  systemProperty("jna.library.path", libPath)
  systemProperty("java.library.path", libPath)
  useJUnit()
}

// Publish to Maven Central via the vanniktech plugin: signs all publications
// and uploads with publishingType=AUTOMATIC, so `publishAndReleaseToMavenCentral`
// auto-releases the Central Portal deployment (the bare `maven-publish` plugin
// can only stage, leaving the artifact unreleased). The Kotlin-specific
// artifactId disambiguates this module from the sibling Java facade in the same
// Maven group; the version is inherited from the top-level `version` above
// (kept current by `alef sync-versions`), so it is omitted from `coordinates`.
mavenPublishing {
  configure(
    KotlinJvm(
      javadocJar = JavadocJar.Empty(),
      sourcesJar = true,
    ),
  )

  publishToMavenCentral()
  signAllPublications()

  coordinates(
    groupId = "dev.spikard",
    artifactId = "spikard-kotlin",
  )

  pom {
    name.set("spikard-kotlin")
    description.set("Rust-centric multi-language HTTP framework with polyglot bindings")
    url.set("https://github.com/Goldziher/spikard")
    licenses {
      license {
        name.set("MIT")
        url.set("https://opensource.org/licenses/MIT")
      }
    }
    developers {
      developer {
        name.set("Na'aman Hirschfeld")
        email.set("nhirschfeld@gmail.com")
      }
    }
    scm {
      url.set("https://github.com/Goldziher/spikard")
      connection.set("scm:git:git://github.com/Goldziher/spikard.git")
      developerConnection.set("scm:git:ssh://git@github.com:Goldziher/spikard.git")
    }
  }
}
